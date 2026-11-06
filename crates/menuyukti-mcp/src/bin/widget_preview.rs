//! Local MCP Apps widget preview server (developer tooling).
//!
//! Serves widget HTML from `assets/` inside an iframe harness with mock
//! `structuredContent`. Heatmap widgets get `assets/shared/widget-theme.css`
//! inlined the same way the MCP server does for ChatGPT.
//!
//! No live-reload: refresh the browser after editing assets. That keeps the
//! preview stable on macOS (FSEvents metadata noise used to reload-loop pages).
//! Does not start the MCP product server.

use std::{
    net::SocketAddr,
    path::{Component, Path, PathBuf},
};

use axum::{
    extract::{Path as AxumPath, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use tracing_subscriber::EnvFilter;

const DEFAULT_PORT: u16 = 8790;
const DEFAULT_HOST: &str = "127.0.0.1";

#[derive(Clone)]
struct PreviewState {
    assets_dir: PathBuf,
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

    let state = PreviewState {
        assets_dir: assets_dir.clone(),
    };

    let port = std::env::var("MENUYUKTI_WIDGET_PREVIEW_PORT")
        .ok()
        .and_then(|raw| raw.trim().parse::<u16>().ok())
        .unwrap_or(DEFAULT_PORT);

    let app = Router::new()
        .route("/", get(index_page))
        .route("/preview.js", get(preview_js))
        .route("/widgets/{*path}", get(serve_widget_asset))
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
    tracing::info!(
        "open http://{DEFAULT_HOST}:{port}/ — refresh the browser after editing assets (no live reload)"
    );

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
        Ok(body) => ([(header::CONTENT_TYPE, content_type)], body).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to read {}: {err}", path.display()),
        )
            .into_response(),
    }
}

async fn serve_widget_asset(
    State(state): State<PreviewState>,
    AxumPath(path): AxumPath<String>,
) -> Response {
    let Some(rel) = sanitize_relative_path(&path) else {
        return (StatusCode::BAD_REQUEST, "invalid widget path").into_response();
    };
    let full = state.assets_dir.join(&rel);
    if !full.starts_with(&state.assets_dir) {
        return (StatusCode::BAD_REQUEST, "invalid widget path").into_response();
    }
    match tokio::fs::read(&full).await {
        Ok(bytes) => {
            let content_type = content_type_for(&full);
            if full.extension().and_then(|e| e.to_str()) == Some("html") {
                match String::from_utf8(bytes) {
                    Ok(html) => {
                        let body = menuyukti_mcp::widget_assets::inline_shared_theme(&html);
                        ([(header::CONTENT_TYPE, content_type)], body).into_response()
                    }
                    Err(err) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("widget HTML is not UTF-8: {err}"),
                    )
                        .into_response(),
                }
            } else {
                ([(header::CONTENT_TYPE, content_type)], bytes).into_response()
            }
        }
        Err(_) => (StatusCode::NOT_FOUND, "widget asset not found").into_response(),
    }
}

fn sanitize_relative_path(raw: &str) -> Option<PathBuf> {
    let path = Path::new(raw);
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => out.push(part),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => return None,
        }
    }
    if out.as_os_str().is_empty() {
        None
    } else {
        Some(out)
    }
}

fn content_type_for(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("svg") => "image/svg+xml",
        _ => "application/octet-stream",
    }
}

fn ensure_dir(path: &Path, label: &str) -> anyhow::Result<()> {
    if path.is_dir() {
        Ok(())
    } else {
        anyhow::bail!("missing {label} directory: {}", path.display())
    }
}
