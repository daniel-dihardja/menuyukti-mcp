//! Local MCP Apps widget preview server (developer tooling).
//!
//! Serves widget HTML from `assets/` inside an iframe harness with mock
//! `structuredContent`, and reloads the browser when watched files change.
//! Does not start the MCP product server.

use std::{
    convert::Infallible,
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};

use axum::{
    extract::State,
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse,
    },
    routing::get,
    Json, Router,
};
use futures_util::stream::{Stream, StreamExt};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::broadcast;
use tokio_stream::wrappers::{errors::BroadcastStreamRecvError, BroadcastStream};
use tower_http::services::ServeDir;
use tracing_subscriber::EnvFilter;

const DEFAULT_PORT: u16 = 8790;
const DEFAULT_HOST: &str = "127.0.0.1";

#[derive(Clone)]
struct PreviewState {
    version: Arc<AtomicU64>,
    reload_tx: broadcast::Sender<()>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let assets_dir = crate_root.join("assets");
    let preview_dir = crate_root.join("preview");
    let fixtures_dir = preview_dir.join("fixtures");

    ensure_dir(&assets_dir, "widget assets")?;
    ensure_dir(&preview_dir, "preview harness")?;
    ensure_dir(&fixtures_dir, "preview fixtures")?;

    let (reload_tx, _) = broadcast::channel(64);
    let state = PreviewState {
        version: Arc::new(AtomicU64::new(0)),
        reload_tx: reload_tx.clone(),
    };

    start_watcher(
        &[assets_dir.clone(), preview_dir.clone()],
        state.version.clone(),
        reload_tx,
    )?;

    let port = std::env::var("MENUYUKTI_WIDGET_PREVIEW_PORT")
        .ok()
        .and_then(|raw| raw.trim().parse::<u16>().ok())
        .unwrap_or(DEFAULT_PORT);

    let app = Router::new()
        .route("/", get(index_page))
        .route("/__watch", get(watch_version))
        .route("/__reload", get(reload_sse))
        .route("/preview.js", get(preview_js))
        .nest_service("/widgets", ServeDir::new(assets_dir.clone()))
        .nest_service("/fixtures", ServeDir::new(fixtures_dir))
        .nest_service("/preview", ServeDir::new(preview_dir.clone()))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(
        "widget preview listening at http://{DEFAULT_HOST}:{port}/ (assets={}, preview={})",
        assets_dir.display(),
        preview_dir.display()
    );
    tracing::info!("open http://{DEFAULT_HOST}:{port}/ then edit files under assets/ or preview/");

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
        })
        .await?;

    Ok(())
}

async fn index_page() -> impl IntoResponse {
    serve_preview_file("preview/index.html", "text/html; charset=utf-8").await
}

async fn preview_js() -> impl IntoResponse {
    serve_preview_file(
        "preview/preview.js",
        "application/javascript; charset=utf-8",
    )
    .await
}

async fn serve_preview_file(relative: &str, content_type: &'static str) -> impl IntoResponse {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    match tokio::fs::read_to_string(&path).await {
        Ok(body) => ([(axum::http::header::CONTENT_TYPE, content_type)], body).into_response(),
        Err(err) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to read {}: {err}", path.display()),
        )
            .into_response(),
    }
}

async fn watch_version(State(state): State<PreviewState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "version": state.version.load(Ordering::Relaxed),
    }))
}

async fn reload_sse(
    State(state): State<PreviewState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.reload_tx.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|result| async move {
        match result {
            Ok(()) => Some(Ok(Event::default().event("reload").data("1"))),
            Err(BroadcastStreamRecvError::Lagged(_)) => {
                Some(Ok(Event::default().event("reload").data("lagged")))
            }
        }
    });
    Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)))
}

fn ensure_dir(path: &Path, label: &str) -> anyhow::Result<()> {
    if path.is_dir() {
        Ok(())
    } else {
        anyhow::bail!("missing {label} directory: {}", path.display())
    }
}

fn start_watcher(
    roots: &[PathBuf],
    version: Arc<AtomicU64>,
    reload_tx: broadcast::Sender<()>,
) -> anyhow::Result<()> {
    let (fs_tx, mut fs_rx) = tokio::sync::mpsc::unbounded_channel();

    let mut watcher = RecommendedWatcher::new(
        move |result: Result<notify::Event, notify::Error>| {
            if let Ok(event) = result {
                let _ = fs_tx.send(event);
            }
        },
        Config::default(),
    )?;

    for root in roots {
        watcher.watch(root, RecursiveMode::Recursive)?;
        tracing::info!("watching {}", root.display());
    }

    // Keep the watcher alive for the process lifetime.
    std::mem::forget(watcher);

    tokio::spawn(async move {
        while let Some(event) = fs_rx.recv().await {
            if !is_interesting(&event.kind) {
                continue;
            }
            // Coalesce bursty FS events from a single save.
            tokio::time::sleep(Duration::from_millis(120)).await;
            while fs_rx.try_recv().is_ok() {}

            let next = version.fetch_add(1, Ordering::Relaxed) + 1;
            tracing::info!("preview assets changed → reload (version {next})");
            let _ = reload_tx.send(());
        }
    });

    Ok(())
}

fn is_interesting(kind: &EventKind) -> bool {
    matches!(
        kind,
        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) | EventKind::Any
    )
}
