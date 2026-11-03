//! Menuyukti MCP server entrypoint (streamable HTTP).

use std::net::SocketAddr;

use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{
        CallToolResult, ContentBlock, Implementation, ProtocolVersion, ServerCapabilities,
        ServerConfig,
    },
    schemars, tool, tool_handler, tool_router,
    transport::streamable_http_server::{
        session::local::LocalSessionManager, StreamableHttpServerConfig, StreamableHttpService,
    },
    ErrorData as McpError, ServerHandler,
};
use serde::Deserialize;
use serde_json::json;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 8787;
const MCP_PATH: &str = "/mcp";

const INSTRUCTIONS: &str = concat!(
    "Local Menuyukti sales analytics for restaurant POS Excel files. ",
    "Call list_workspaces to see analysis workspaces and their Excel files, ",
    "summarize_local_sales for aggregates, list_normalized_sales ",
    "for plain normalized line items, or get_menu_heatmaps for per-menu ",
    "daily/weekly demand patterns. Paths must be under MENUYUKTI_DATA_DIR. ",
    "Files stay on the local machine."
);

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct PathArgs {
    /// Relative or absolute path to a POS Excel file under MENUYUKTI_DATA_DIR.
    path: String,
}

#[derive(Clone)]
struct MenuyuktiMcp {
    // Populated by `#[tool_router]`; read by generated `ServerHandler` glue.
    #[allow(dead_code)]
    tool_router: ToolRouter<Self>,
}

impl MenuyuktiMcp {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

fn text_json(value: impl serde::Serialize) -> CallToolResult {
    CallToolResult::success(vec![ContentBlock::text(
        serde_json::to_string(&value).unwrap_or_else(|err| {
            json!({ "error": format!("failed to serialize response: {err}") }).to_string()
        }),
    )])
}

fn error_json(message: impl Into<String>) -> CallToolResult {
    text_json(json!({ "error": message.into() }))
}

fn load_or_error(path: &str) -> Result<menuyukti::NormalizedSales, CallToolResult> {
    menuyukti::load_normalized_sales(path, None).map_err(|err| error_json(err.to_string()))
}

#[tool_router]
impl MenuyuktiMcp {
    #[tool(
        description = "List analysis workspaces under MENUYUKTI_DATA_DIR (typically workspaces/). Groups Excel files by workspace folder and returns relative file paths you can pass to the other sales tools. Ignores non-Excel files."
    )]
    async fn list_workspaces(&self) -> Result<CallToolResult, McpError> {
        match menuyukti::list_workspaces(None) {
            Ok(listed) => Ok(text_json(listed)),
            Err(err) => Ok(error_json(err.to_string())),
        }
    }

    #[tool(
        description = "Summarize a local POS Excel sales export. Reads the file only when path resolves inside MENUYUKTI_DATA_DIR. Returns detected POS, period, revenue, quantity, and a capped top-item list."
    )]
    async fn summarize_local_sales(
        &self,
        Parameters(PathArgs { path }): Parameters<PathArgs>,
    ) -> Result<CallToolResult, McpError> {
        let loaded = match load_or_error(&path) {
            Ok(v) => v,
            Err(resp) => return Ok(resp),
        };
        match menuyukti::summarize_sales(loaded.pos, &loaded.rows) {
            Ok(summary) => Ok(text_json(summary)),
            Err(err) => Ok(error_json(err)),
        }
    }

    #[tool(
        description = "List plain normalized line items from a local POS Excel sales export. Reads the file only when path resolves inside MENUYUKTI_DATA_DIR. Returns detected POS and the full cleaned row list (no aggregates)."
    )]
    async fn list_normalized_sales(
        &self,
        Parameters(PathArgs { path }): Parameters<PathArgs>,
    ) -> Result<CallToolResult, McpError> {
        let loaded = match load_or_error(&path) {
            Ok(v) => v,
            Err(resp) => return Ok(resp),
        };
        Ok(text_json(menuyukti::list_normalized_rows(
            loaded.pos,
            &loaded.rows,
        )))
    }

    #[tool(
        description = "Return per-menu daily (hourly) and weekly demand heatmaps from a local POS Excel sales export. Reads the file only when path resolves inside MENUYUKTI_DATA_DIR. Useful for when each dish sells most."
    )]
    async fn get_menu_heatmaps(
        &self,
        Parameters(PathArgs { path }): Parameters<PathArgs>,
    ) -> Result<CallToolResult, McpError> {
        let loaded = match load_or_error(&path) {
            Ok(v) => v,
            Err(resp) => return Ok(resp),
        };
        match menuyukti::calculate_menu_heatmaps(&loaded.rows) {
            Ok(heatmaps) => Ok(text_json(json!({
                "pos": loaded.pos,
                "menu_count": heatmaps.len(),
                "heatmaps": heatmaps,
            }))),
            Err(err) => Ok(error_json(err)),
        }
    }
}

#[tool_handler]
impl ServerHandler for MenuyuktiMcp {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new(
                "menuyukti-mcp",
                env!("CARGO_PKG_VERSION"),
            ))
            .with_protocol_version(ProtocolVersion::default())
            .with_instructions(INSTRUCTIONS.to_string())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let ct = tokio_util::sync::CancellationToken::new();
    let config = StreamableHttpServerConfig::default()
        .with_legacy_session_mode(false)
        .with_json_response(true)
        .with_cancellation_token(ct.child_token());

    let service = StreamableHttpService::new(
        || Ok(MenuyuktiMcp::new()),
        LocalSessionManager::default().into(),
        config,
    );

    let router = axum::Router::new().nest_service(MCP_PATH, service);
    let addr = SocketAddr::from(([127, 0, 0, 1], DEFAULT_PORT));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!(
        "menuyukti-mcp MCP listening at http://{DEFAULT_HOST}:{DEFAULT_PORT}{MCP_PATH} (menuyukti {})",
        menuyukti::VERSION
    );

    axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            tokio::signal::ctrl_c().await.ok();
            ct.cancel();
        })
        .await?;

    Ok(())
}
