//! Menuyukti MCP server entrypoint (streamable HTTP).

use std::net::SocketAddr;

use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{
        CallToolResult, ContentBlock, Implementation, ListResourcesResult, MetaObject,
        ProtocolVersion, ReadResourceRequestParams, ReadResourceResult, Resource, ResourceContents,
        ServerCapabilities, ServerConfig,
    },
    schemars, tool, tool_handler, tool_router,
    transport::streamable_http_server::{
        session::local::LocalSessionManager, StreamableHttpServerConfig, StreamableHttpService,
    },
    ErrorData as McpError, ServerHandler,
};
use serde::Deserialize;
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 8787;
const MCP_PATH: &str = "/mcp";

/// MCP Apps UI resource for the hello-world smoke widget.
/// Bump the URI when the HTML changes so ChatGPT does not keep a cached blank template.
const HELLO_WORLD_WIDGET_URI: &str = "ui://widget/hello-world-v3.html";
/// ChatGPT Apps SDK examples still serve widgets as `text/html+skybridge`.
/// Keep the MCP Apps profile MIME as a secondary compatibility note in docs/tests.
const WIDGET_MIME_TYPE: &str = "text/html+skybridge";
const HELLO_WORLD_WIDGET_HTML: &str = include_str!("../assets/hello_world_widget.html");

const INSTRUCTIONS: &str = concat!(
    "Local Menuyukti MCP sales analytics for restaurant POS Excel files. ",
    "Onboarding: when an owner drops sales (+ optional COGS) Excel into a new ",
    "workspace folder, call list_workspace_files, infer file roles, confirm if ",
    "unclear, then write_workspace_config (workspace.yaml). After that, call ",
    "summarize_local_sales with the workspace folder path. ",
    "Menu engineering (analyze_menu_engineering) requires a workspace folder ",
    "with both files.sales and files.cogs configured; if COGS is missing, ask ",
    "the owner for a unit-cost Excel, list_workspace_files, then write_workspace_config. ",
    "Weak items review (get_weak_items_review) also needs sales + COGS; use it for ",
    "reprice / improve / remove decisions on low-popularity, low-margin dishes. ",
    "Pair lift (get_pair_lift_matrix) returns co-purchase lift for combo ideas; ",
    "sales alone is enough (top-by-presence focus). When files.cogs is configured ",
    "and menu engineering yields at least two stars, those stars become the focus. ",
    "Venue slot strength (get_venue_slot_strength) returns weekday × meal-period ",
    "demand for timing/staffing; sales alone is enough. ",
    "Location notes: keep venue facts (hours, seating, cuisine, constraints, ",
    "past decisions) in knowledge.md via read_workspace_knowledge / ",
    "write_workspace_knowledge. Read before rewriting so updates replace the ",
    "full document without losing prior notes. ",
    "Also available: list_workspaces for Excel inventory, list_normalized_sales ",
    "for cleaned line items, get_menu_heatmaps for per-menu demand patterns. ",
    "UI smoke only: render_hello_world_widget shows a Hello World MCP Apps widget ",
    "with hard-coded mock data (no sales files). Do not use it for analytics. ",
    "Paths must stay under MENUYUKTI_DATA_DIR. Files stay on the local machine."
);

fn hello_world_widget_meta() -> MetaObject {
    let mut meta = MetaObject::new();
    meta.insert(
        "ui".to_string(),
        json!({ "resourceUri": HELLO_WORLD_WIDGET_URI }),
    );
    // ChatGPT compatibility aliases / hints used by Apps SDK hosts.
    meta.insert(
        "openai/outputTemplate".to_string(),
        json!(HELLO_WORLD_WIDGET_URI),
    );
    meta.insert("openai/widgetAccessible".to_string(), json!(true));
    meta.insert(
        "openai/toolInvocation/invoking".to_string(),
        json!("Rendering Hello World widget…"),
    );
    meta.insert(
        "openai/toolInvocation/invoked".to_string(),
        json!("Hello World widget ready."),
    );
    meta
}

fn hello_world_tool_result() -> CallToolResult {
    // Hosts that mount then tear down widgets often require the template URI on
    // the *result* `_meta`, not only on tools/list.
    let mut result = CallToolResult::structured(mock_hello_world_payload())
        .with_meta(Some(hello_world_widget_meta()));
    result.content = vec![ContentBlock::text(
        "Showing the Hello World MCP Apps widget (mock data).",
    )];
    result
}

fn hello_world_resource() -> Resource {
    Resource::new(HELLO_WORLD_WIDGET_URI, "hello-world-widget")
        .with_title("Hello World widget")
        .with_description("MCP Apps smoke widget that renders mock Hello World structuredContent.")
        .with_mime_type(WIDGET_MIME_TYPE)
        .with_meta(hello_world_widget_meta())
}

fn hello_world_resource_contents() -> ResourceContents {
    ResourceContents::text(HELLO_WORLD_WIDGET_HTML, HELLO_WORLD_WIDGET_URI)
        .with_mime_type(WIDGET_MIME_TYPE)
        .with_meta(hello_world_widget_meta())
}

fn mock_hello_world_payload() -> serde_json::Value {
    json!({
        "message": "Hello World",
        "source": "mock",
    })
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct PathArgs {
    /// Workspace folder (uses workspace.yaml sales role) or Excel file under MENUYUKTI_DATA_DIR.
    path: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct WorkspacePathArgs {
    /// Relative or absolute path to a workspace directory under MENUYUKTI_DATA_DIR.
    path: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct WorkspacePeriodArgs {
    #[serde(default)]
    label: Option<String>,
    #[serde(default)]
    start: Option<String>,
    #[serde(default)]
    end: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct WorkspaceFileRoleArgs {
    /// Path relative to the workspace folder (for example sales.xlsx).
    path: String,
    #[serde(default)]
    description: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct WorkspaceFilesArgs {
    sales: WorkspaceFileRoleArgs,
    #[serde(default)]
    cogs: Option<WorkspaceFileRoleArgs>,
    #[serde(default)]
    knowledge: Option<WorkspaceFileRoleArgs>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct WorkspaceConfigArgs {
    /// Config schema version (must be 1).
    version: u32,
    name: String,
    venue: String,
    #[serde(default)]
    period: Option<WorkspacePeriodArgs>,
    #[serde(default)]
    currency: Option<String>,
    files: WorkspaceFilesArgs,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct WriteWorkspaceConfigArgs {
    /// Workspace folder under MENUYUKTI_DATA_DIR (for example my-venue-aug-2026).
    path: String,
    config: WorkspaceConfigArgs,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct WriteWorkspaceKnowledgeArgs {
    /// Workspace folder under MENUYUKTI_DATA_DIR (for example my-venue-aug-2026).
    path: String,
    /// Full markdown body to store in knowledge.md (full replace; read first).
    content: String,
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

/// Prefer menu-engineering stars as focus when workspace COGS is available.
fn optional_star_focus(path: &str, rows: &[menuyukti::LineItem]) -> Option<Vec<String>> {
    let cogs = menuyukti::load_workspace_cogs(path, None).ok()?;
    let matrix = menuyukti::compute_menu_engineering_from_orders(rows, &cogs.unit_costs).ok()?;
    let stars: Vec<String> = matrix
        .items
        .into_iter()
        .filter(|item| item.category == "star")
        .map(|item| item.menu)
        .collect();
    if stars.len() >= 2 {
        Some(stars)
    } else {
        None
    }
}

fn role_from_args(role: WorkspaceFileRoleArgs) -> menuyukti::WorkspaceFileRole {
    menuyukti::WorkspaceFileRole {
        path: role.path,
        description: role.description,
    }
}

fn config_from_args(args: WorkspaceConfigArgs) -> menuyukti::WorkspaceConfig {
    menuyukti::WorkspaceConfig {
        version: args.version,
        name: args.name,
        venue: args.venue,
        period: args.period.map(|p| menuyukti::WorkspacePeriod {
            label: p.label,
            start: p.start,
            end: p.end,
        }),
        currency: args.currency,
        files: menuyukti::WorkspaceFiles {
            sales: role_from_args(args.files.sales),
            cogs: args.files.cogs.map(role_from_args),
            knowledge: args.files.knowledge.map(role_from_args),
        },
    }
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
        description = "List all files in one workspace directory under MENUYUKTI_DATA_DIR (Excel, workspace.yaml, notes, and other docs). Path must be a workspace folder (for example demo-cafe-july-2026). Skips Excel lock files (~$*). Use this first when onboarding a new folder."
    )]
    async fn list_workspace_files(
        &self,
        Parameters(WorkspacePathArgs { path }): Parameters<WorkspacePathArgs>,
    ) -> Result<CallToolResult, McpError> {
        match menuyukti::list_workspace_files(&path, None) {
            Ok(listed) => Ok(text_json(listed)),
            Err(err) => Ok(error_json(err.to_string())),
        }
    }

    #[tool(
        description = "Write workspace.yaml for a workspace folder under MENUYUKTI_DATA_DIR. Use after list_workspace_files when onboarding: set sales/cogs roles, venue, period, and currency. Overwrites an existing config. Role paths must be relative to the workspace folder."
    )]
    async fn write_workspace_config(
        &self,
        Parameters(WriteWorkspaceConfigArgs { path, config }): Parameters<WriteWorkspaceConfigArgs>,
    ) -> Result<CallToolResult, McpError> {
        let config = config_from_args(config);
        match menuyukti::write_workspace_config(&path, &config, None) {
            Ok(written) => Ok(text_json(written)),
            Err(err) => Ok(error_json(err.to_string())),
        }
    }

    #[tool(
        description = "Read knowledge.md (location/venue notes) for a workspace folder under MENUYUKTI_DATA_DIR. Uses files.knowledge when set, otherwise knowledge.md. Missing file returns exists=false with empty content. Call this before write_workspace_knowledge so rewrites keep prior notes."
    )]
    async fn read_workspace_knowledge(
        &self,
        Parameters(WorkspacePathArgs { path }): Parameters<WorkspacePathArgs>,
    ) -> Result<CallToolResult, McpError> {
        match menuyukti::read_workspace_knowledge(&path, None) {
            Ok(read) => Ok(text_json(read)),
            Err(err) => Ok(error_json(err.to_string())),
        }
    }

    #[tool(
        description = "Create or fully replace knowledge.md for a workspace folder under MENUYUKTI_DATA_DIR. Store location/venue facts (hours, seating, cuisine, constraints, past decisions). Full replace — call read_workspace_knowledge first. When workspace.yaml exists without files.knowledge, sets that role to knowledge.md."
    )]
    async fn write_workspace_knowledge(
        &self,
        Parameters(WriteWorkspaceKnowledgeArgs { path, content }): Parameters<
            WriteWorkspaceKnowledgeArgs,
        >,
    ) -> Result<CallToolResult, McpError> {
        match menuyukti::write_workspace_knowledge(&path, &content, None) {
            Ok(written) => Ok(text_json(written)),
            Err(err) => Ok(error_json(err.to_string())),
        }
    }

    #[tool(
        description = "Summarize local POS sales. Path may be a workspace folder (reads files.sales from workspace.yaml) or an Excel file under MENUYUKTI_DATA_DIR. Returns detected POS, period, revenue, quantity, and a capped top-item list."
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
        description = "List plain normalized line items from local POS sales. Path may be a workspace folder (workspace.yaml sales role) or an Excel file under MENUYUKTI_DATA_DIR. Returns detected POS and the full cleaned row list (no aggregates)."
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
        description = "Return per-menu daily (hourly) and weekly demand heatmaps from local POS sales. Path may be a workspace folder (workspace.yaml sales role) or an Excel file under MENUYUKTI_DATA_DIR. Useful for when each dish sells most."
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

    #[tool(
        description = "Return venue slot strength: weekday × meal-period demand (order counts, traffic share, demand index, relative demand). Path may be a workspace folder (workspace.yaml sales role) or an Excel file under MENUYUKTI_DATA_DIR. Sales alone is enough — use for timing, staffing, and when to promote. Returns a fixed 35-cell slot_demand_profile."
    )]
    async fn get_venue_slot_strength(
        &self,
        Parameters(PathArgs { path }): Parameters<PathArgs>,
    ) -> Result<CallToolResult, McpError> {
        let loaded = match load_or_error(&path) {
            Ok(v) => v,
            Err(resp) => return Ok(resp),
        };
        let profile = menuyukti::compute_slot_demand_profile_from_orders(&loaded.rows);
        Ok(text_json(json!({
            "pos": loaded.pos,
            "slot_count": profile.len(),
            "slot_demand_profile": profile,
        })))
    }

    #[tool(
        description = "Compute the menu engineering matrix (star / plow_horse / puzzle / weak) for a workspace folder under MENUYUKTI_DATA_DIR. Requires workspace.yaml with both files.sales and files.cogs. Returns thresholds, quadrant distribution, per-item classifications, and COGS coverage. If COGS is missing, list_workspace_files / write_workspace_config or ask the owner for unit costs first."
    )]
    async fn analyze_menu_engineering(
        &self,
        Parameters(WorkspacePathArgs { path }): Parameters<WorkspacePathArgs>,
    ) -> Result<CallToolResult, McpError> {
        let cogs = match menuyukti::load_workspace_cogs(&path, None) {
            Ok(v) => v,
            Err(err) => {
                return Ok(error_json(format!(
                    "{err}. Menu engineering needs files.cogs on the workspace: \
                     drop a COGS Excel, call list_workspace_files, then write_workspace_config \
                     (or ask the owner for unit costs)."
                )));
            }
        };
        let loaded = match load_or_error(&path) {
            Ok(v) => v,
            Err(resp) => return Ok(resp),
        };
        match menuyukti::compute_menu_engineering_from_orders(&loaded.rows, &cogs.unit_costs) {
            Ok(matrix) => Ok(text_json(json!({
                "pos": loaded.pos,
                "workspace": path,
                "cogs_path": cogs.path,
                "thresholds": matrix.thresholds,
                "distribution": matrix.distribution,
                "items": matrix.items,
                "coverage": matrix.coverage,
            }))),
            Err(err) => Ok(error_json(format!(
                "{err}. Fix COGS coverage (Menu + COGS columns matching sold menu names) \
                 or ask the owner to update unit costs."
            ))),
        }
    }

    #[tool(
        description = "Return Weak items review analytics for reprice / improve / remove decisions. Workspace folder under MENUYUKTI_DATA_DIR with files.sales and files.cogs required. Filters the menu-engineering matrix to the weak quadrant and adds period depth (avg qty/day, portfolio item/margin share). If COGS is missing, list_workspace_files / write_workspace_config or ask the owner for unit costs first."
    )]
    async fn get_weak_items_review(
        &self,
        Parameters(WorkspacePathArgs { path }): Parameters<WorkspacePathArgs>,
    ) -> Result<CallToolResult, McpError> {
        let cogs = match menuyukti::load_workspace_cogs(&path, None) {
            Ok(v) => v,
            Err(err) => {
                return Ok(error_json(format!(
                    "{err}. Weak items review needs files.cogs on the workspace: \
                     drop a COGS Excel, call list_workspace_files, then write_workspace_config \
                     (or ask the owner for unit costs)."
                )));
            }
        };
        let loaded = match load_or_error(&path) {
            Ok(v) => v,
            Err(resp) => return Ok(resp),
        };
        match menuyukti::compute_menu_engineering_from_orders(&loaded.rows, &cogs.unit_costs) {
            Ok(matrix) => {
                let review = menuyukti::adapt_weak_review_from_orders(&matrix, &loaded.rows);
                Ok(text_json(json!({
                    "pos": loaded.pos,
                    "workspace": path,
                    "cogs_path": cogs.path,
                    "period_start": review.period_start,
                    "period_end": review.period_end,
                    "stats": review.stats,
                    "distribution_stats": review.distribution_stats,
                    "items": review.rows,
                    "coverage": matrix.coverage,
                })))
            }
            Err(err) => Ok(error_json(format!(
                "{err}. Fix COGS coverage (Menu + COGS columns matching sold menu names) \
                 or ask the owner to update unit costs."
            ))),
        }
    }

    #[tool(
        description = "Return the pair lift matrix (co-purchase affinity) for combo and upsell ideas. Path may be a workspace folder (workspace.yaml sales role) or an Excel file under MENUYUKTI_DATA_DIR. Sales alone is enough (top menus by order presence). When the path is a workspace with files.cogs and menu engineering yields at least two stars, those stars become the focus. Returns focus_menus, matrix_lift, totals, scope, and ranked pairs."
    )]
    async fn get_pair_lift_matrix(
        &self,
        Parameters(PathArgs { path }): Parameters<PathArgs>,
    ) -> Result<CallToolResult, McpError> {
        let loaded = match load_or_error(&path) {
            Ok(v) => v,
            Err(resp) => return Ok(resp),
        };
        let star_focus = optional_star_focus(&path, &loaded.rows);
        let affinities = menuyukti::compute_menu_basket_affinities_from_orders(
            &loaded.rows,
            star_focus.as_deref(),
        );
        Ok(text_json(json!({
            "pos": loaded.pos,
            "total_orders": affinities.total_orders,
            "multi_item_order_count": affinities.multi_item_order_count,
            "avg_distinct_items_per_order": affinities.avg_distinct_items_per_order,
            "scope": affinities.scope,
            "focus_menus": affinities.focus_menus,
            "matrix_lift": affinities.matrix_lift,
            "pairs": affinities.pairs,
        })))
    }

    #[tool(
        description = "Render a Hello World MCP Apps widget using hard-coded mock data. UI smoke / wiring proof only — does not read sales files. Prefer analytics tools for real venue questions.",
        annotations(title = "Render Hello World widget", read_only_hint = true),
        meta = hello_world_widget_meta()
    )]
    async fn render_hello_world_widget(&self) -> Result<CallToolResult, McpError> {
        Ok(hello_world_tool_result())
    }
}

#[tool_handler]
impl ServerHandler for MenuyuktiMcp {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .build(),
        )
        .with_server_info(Implementation::new(
            "menuyukti-mcp",
            env!("CARGO_PKG_VERSION"),
        ))
        .with_protocol_version(ProtocolVersion::default())
        .with_instructions(INSTRUCTIONS.to_string())
    }

    async fn list_resources(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        Ok(ListResourcesResult {
            resources: vec![hello_world_resource()],
            ..Default::default()
        })
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::ReadResourceResponse, McpError> {
        if request.uri != HELLO_WORLD_WIDGET_URI {
            return Err(McpError::resource_not_found(
                format!("unknown resource URI: {}", request.uri),
                Some(json!({ "uri": request.uri })),
            ));
        }
        Ok(ReadResourceResult::new(vec![hello_world_resource_contents()]).into())
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

    let port = std::env::var("MENUYUKTI_MCP_PORT")
        .ok()
        .and_then(|raw| raw.trim().parse::<u16>().ok())
        .unwrap_or(DEFAULT_PORT);
    // ChatGPT web/sandbox iframes call the MCP HTTP endpoint cross-origin.
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let router = axum::Router::new()
        .nest_service(MCP_PATH, service)
        .layer(cors);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!(
        "menuyukti-mcp MCP listening at http://{DEFAULT_HOST}:{port}{MCP_PATH} (menuyukti {})",
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
