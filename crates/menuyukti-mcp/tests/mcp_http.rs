use std::fs;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::{json, Value};

fn port_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

fn free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

struct RunningServer {
    child: Child,
    addr: SocketAddr,
}

impl Drop for RunningServer {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn start_server(data_dir: &PathBuf) -> RunningServer {
    let port = free_port();
    let addr: SocketAddr = format!("127.0.0.1:{port}").parse().unwrap();

    let bin = env!("CARGO_BIN_EXE_menuyukti-mcp");
    let mut child = Command::new(bin)
        .env("MENUYUKTI_DATA_DIR", data_dir)
        .env("MENUYUKTI_MCP_PORT", port.to_string())
        .env("RUST_LOG", "error")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn menuyukti-mcp");

    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        if TcpStream::connect_timeout(&addr, Duration::from_millis(100)).is_ok() {
            return RunningServer { child, addr };
        }
        thread::sleep(Duration::from_millis(50));
    }
    let _ = child.kill();
    let _ = child.wait();
    panic!("menuyukti-mcp failed to listen on {addr}");
}

fn http_post_json(addr: SocketAddr, body: Value) -> Value {
    let payload = serde_json::to_vec(&body).unwrap();
    let mut stream = TcpStream::connect(addr).expect("connect mcp");
    stream
        .set_read_timeout(Some(Duration::from_secs(30)))
        .unwrap();
    let request = format!(
        "POST /mcp HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nAccept: application/json, text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        addr,
        payload.len()
    );
    stream.write_all(request.as_bytes()).unwrap();
    stream.write_all(&payload).unwrap();

    let mut raw = Vec::new();
    stream.read_to_end(&mut raw).unwrap();
    let text = String::from_utf8_lossy(&raw);
    let body = text
        .split("\r\n\r\n")
        .nth(1)
        .unwrap_or("")
        .trim()
        .to_string();
    serde_json::from_str(&body).unwrap_or_else(|err| {
        panic!("invalid JSON from MCP HTTP response: {err}; body={body}");
    })
}

fn tool_payload(addr: SocketAddr, name: &str, arguments: Value) -> Value {
    let response = http_post_json(
        addr,
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": { "name": name, "arguments": arguments }
        }),
    );
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .unwrap_or_else(|| panic!("tool={name} missing text content: {response}"));
    serde_json::from_str(text).unwrap_or_else(|err| {
        panic!("tool={name} returned non-JSON text: {err}; text={text}");
    })
}

#[test]
fn mcp_http_lists_tools_and_summarizes_demo() {
    let _guard = port_lock().lock().unwrap();
    let data_dir = repo_root().join("workspaces");
    let server = start_server(&data_dir);

    let listed_tools = http_post_json(
        server.addr,
        json!({"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}),
    );
    let names: Vec<&str> = listed_tools["result"]["tools"]
        .as_array()
        .expect("tools array")
        .iter()
        .filter_map(|t| t["name"].as_str())
        .collect();
    for required in [
        "get_setup_status",
        "list_workspaces",
        "list_workspace_files",
        "write_workspace_config",
        "read_workspace_knowledge",
        "write_workspace_knowledge",
        "summarize_local_sales",
        "list_normalized_sales",
        "get_menu_heatmaps",
        "get_venue_slot_strength",
        "analyze_menu_engineering",
        "get_weak_items_review",
        "get_pair_lift_matrix",
        "render_hello_world_widget",
        "render_venue_slot_strength",
        "render_pair_lift_matrix",
        "render_menu_heatmaps",
    ] {
        assert!(
            names.contains(&required),
            "tools/list missing {required}; got {names:?}"
        );
    }

    let tools = listed_tools["result"]["tools"]
        .as_array()
        .expect("tools array");
    let hello_tool = tools
        .iter()
        .find(|t| t["name"] == "render_hello_world_widget")
        .expect("render_hello_world_widget tool entry");
    assert_eq!(
        hello_tool["_meta"]["ui"]["resourceUri"].as_str(),
        Some("ui://widget/hello-world-v3.html"),
        "render_hello_world_widget missing _meta.ui.resourceUri; tool={hello_tool}"
    );
    assert_eq!(
        hello_tool["_meta"]["openai/outputTemplate"].as_str(),
        Some("ui://widget/hello-world-v3.html"),
        "render_hello_world_widget missing openai/outputTemplate; tool={hello_tool}"
    );
    let venue_render = tools
        .iter()
        .find(|t| t["name"] == "render_venue_slot_strength")
        .expect("render_venue_slot_strength tool entry");
    assert_eq!(
        venue_render["_meta"]["ui"]["resourceUri"].as_str(),
        Some("ui://widget/venue-slot-strength-v6.html"),
        "render_venue_slot_strength missing _meta.ui.resourceUri; tool={venue_render}"
    );
    assert_eq!(
        venue_render["_meta"]["openai/outputTemplate"].as_str(),
        Some("ui://widget/venue-slot-strength-v6.html"),
        "render_venue_slot_strength missing openai/outputTemplate; tool={venue_render}"
    );
    assert_eq!(
        venue_render["_meta"]["ui"]["prefersBorder"].as_bool(),
        Some(true),
        "render_venue_slot_strength should prefer host border/padding; tool={venue_render}"
    );
    assert_eq!(
        venue_render["_meta"]["openai/widgetPrefersBorder"].as_bool(),
        Some(true),
        "render_venue_slot_strength missing openai/widgetPrefersBorder; tool={venue_render}"
    );
    let pair_render = tools
        .iter()
        .find(|t| t["name"] == "render_pair_lift_matrix")
        .expect("render_pair_lift_matrix tool entry");
    assert_eq!(
        pair_render["_meta"]["ui"]["resourceUri"].as_str(),
        Some("ui://widget/pair-lift-matrix-v5.html"),
        "render_pair_lift_matrix missing _meta.ui.resourceUri; tool={pair_render}"
    );
    assert_eq!(
        pair_render["_meta"]["openai/outputTemplate"].as_str(),
        Some("ui://widget/pair-lift-matrix-v5.html"),
        "render_pair_lift_matrix missing openai/outputTemplate; tool={pair_render}"
    );
    assert_eq!(
        pair_render["_meta"]["ui"]["prefersBorder"].as_bool(),
        Some(true),
        "render_pair_lift_matrix should prefer host border/padding; tool={pair_render}"
    );
    assert_eq!(
        pair_render["_meta"]["openai/widgetPrefersBorder"].as_bool(),
        Some(true),
        "render_pair_lift_matrix missing openai/widgetPrefersBorder; tool={pair_render}"
    );
    let menu_render = tools
        .iter()
        .find(|t| t["name"] == "render_menu_heatmaps")
        .expect("render_menu_heatmaps tool entry");
    assert_eq!(
        menu_render["_meta"]["ui"]["resourceUri"].as_str(),
        Some("ui://widget/menu-heatmaps-v2.html"),
        "render_menu_heatmaps missing _meta.ui.resourceUri; tool={menu_render}"
    );
    assert_eq!(
        menu_render["_meta"]["openai/outputTemplate"].as_str(),
        Some("ui://widget/menu-heatmaps-v2.html"),
        "render_menu_heatmaps missing openai/outputTemplate; tool={menu_render}"
    );
    assert_eq!(
        menu_render["_meta"]["ui"]["prefersBorder"].as_bool(),
        Some(true),
        "render_menu_heatmaps should prefer host border/padding; tool={menu_render}"
    );
    assert_eq!(
        menu_render["_meta"]["openai/widgetPrefersBorder"].as_bool(),
        Some(true),
        "render_menu_heatmaps missing openai/widgetPrefersBorder; tool={menu_render}"
    );
    for analytics in [
        "summarize_local_sales",
        "get_menu_heatmaps",
        "get_venue_slot_strength",
        "get_pair_lift_matrix",
        "analyze_menu_engineering",
        "get_weak_items_review",
    ] {
        let tool = tools
            .iter()
            .find(|t| t["name"] == analytics)
            .unwrap_or_else(|| panic!("missing analytics tool {analytics}"));
        assert!(
            tool.get("_meta").and_then(|m| m.get("ui")).is_none(),
            "analytics tool {analytics} must not declare UI widget meta; tool={tool}"
        );
    }

    let workspaces = tool_payload(server.addr, "list_workspaces", json!({}));
    assert!(
        workspaces["workspace_count"].as_u64().unwrap_or(0) >= 1,
        "tool=list_workspaces unexpected payload={workspaces}"
    );

    let workspace_files = tool_payload(
        server.addr,
        "list_workspace_files",
        json!({"path": "demo-cafe-july-2026"}),
    );
    assert!(
        workspace_files["file_count"].as_u64().unwrap_or(0) >= 3,
        "tool=list_workspace_files unexpected payload={workspace_files}"
    );
    let file_paths: Vec<&str> = workspace_files["files"]
        .as_array()
        .expect("files array")
        .iter()
        .filter_map(|f| f["path"].as_str())
        .collect();
    assert!(
        file_paths.contains(&"demo-cafe-july-2026/workspace.yaml"),
        "tool=list_workspace_files missing workspace.yaml; files={file_paths:?}"
    );

    let summary = tool_payload(
        server.addr,
        "summarize_local_sales",
        json!({"path": "demo-cafe-july-2026/sales.xlsx"}),
    );
    assert_eq!(
        summary["pos"], "esb",
        "tool=summarize_local_sales fixture=demo-cafe payload={summary}"
    );
    assert_eq!(
        summary["total_items_sold"], 1748,
        "tool=summarize_local_sales fixture=demo-cafe field=total_items_sold payload={summary}"
    );

    let summary_via_workspace = tool_payload(
        server.addr,
        "summarize_local_sales",
        json!({"path": "demo-cafe-july-2026"}),
    );
    assert_eq!(
        summary_via_workspace["pos"], "esb",
        "tool=summarize_local_sales workspace folder payload={summary_via_workspace}"
    );

    let matrix = tool_payload(
        server.addr,
        "analyze_menu_engineering",
        json!({"path": "demo-cafe-july-2026"}),
    );
    assert!(
        matrix.get("error").is_none(),
        "tool=analyze_menu_engineering demo cafe error payload={matrix}"
    );
    assert!(
        matrix["items"]
            .as_array()
            .map(|a| !a.is_empty())
            .unwrap_or(false),
        "tool=analyze_menu_engineering expected items; payload={matrix}"
    );
    assert!(
        matrix["thresholds"]["avg_popularity"]
            .as_f64()
            .unwrap_or(0.0)
            > 0.0,
        "tool=analyze_menu_engineering thresholds; payload={matrix}"
    );

    let weak = tool_payload(
        server.addr,
        "get_weak_items_review",
        json!({"path": "demo-cafe-july-2026"}),
    );
    assert!(
        weak.get("error").is_none(),
        "tool=get_weak_items_review demo cafe error payload={weak}"
    );
    assert!(
        weak["items"]
            .as_array()
            .map(|a| !a.is_empty())
            .unwrap_or(false),
        "tool=get_weak_items_review expected weak items; payload={weak}"
    );
    assert!(
        weak["stats"]["item_count"].as_u64().unwrap_or(0) >= 1,
        "tool=get_weak_items_review stats.item_count; payload={weak}"
    );
    assert!(
        weak["stats"]["period_days"].as_i64().unwrap_or(0) > 0,
        "tool=get_weak_items_review expected period_days; payload={weak}"
    );
    assert!(
        weak["period_start"].as_str().is_some(),
        "tool=get_weak_items_review period_start; payload={weak}"
    );

    let heatmaps = tool_payload(
        server.addr,
        "get_menu_heatmaps",
        json!({"path": "demo-cafe-july-2026"}),
    );
    assert!(
        heatmaps.get("error").is_none(),
        "tool=get_menu_heatmaps demo cafe error payload={heatmaps}"
    );
    let heatmap_items = heatmaps["heatmaps"].as_array().expect("heatmaps array");
    assert!(
        !heatmap_items.is_empty(),
        "tool=get_menu_heatmaps expected items; payload={heatmaps}"
    );
    let matrix_by_menu: std::collections::HashMap<&str, &str> = matrix["items"]
        .as_array()
        .expect("matrix items")
        .iter()
        .filter_map(|item| Some((item["menu"].as_str()?, item["category"].as_str()?)))
        .collect();
    let mut enriched = 0;
    for item in heatmap_items {
        let menu = item["menu"].as_str().expect("heatmap menu");
        let category = item["matrix_category"].as_str();
        if let Some(expected) = matrix_by_menu.get(menu) {
            assert_eq!(
                category,
                Some(*expected),
                "tool=get_menu_heatmaps menu={menu} matrix_category mismatch; item={item}"
            );
            enriched += 1;
        }
    }
    assert_eq!(
        enriched,
        matrix_by_menu.len(),
        "tool=get_menu_heatmaps expected every matrix menu enriched; payload={heatmaps}"
    );

    let heatmaps_sales_only = tool_payload(
        server.addr,
        "get_menu_heatmaps",
        json!({"path": "demo-cafe-july-2026/sales.xlsx"}),
    );
    assert!(
        heatmaps_sales_only.get("error").is_none(),
        "tool=get_menu_heatmaps sales Excel should succeed without COGS; payload={heatmaps_sales_only}"
    );
    let sales_only_items = heatmaps_sales_only["heatmaps"]
        .as_array()
        .expect("sales-only heatmaps array");
    assert!(
        !sales_only_items.is_empty(),
        "tool=get_menu_heatmaps sales Excel expected items; payload={heatmaps_sales_only}"
    );
    assert!(
        sales_only_items
            .iter()
            .all(|item| item.get("matrix_category").is_none()
                || item["matrix_category"].is_null()),
        "tool=get_menu_heatmaps sales Excel must leave matrix_category null; payload={heatmaps_sales_only}"
    );

    let slots = tool_payload(
        server.addr,
        "get_venue_slot_strength",
        json!({"path": "demo-cafe-july-2026"}),
    );
    assert!(
        slots.get("error").is_none(),
        "tool=get_venue_slot_strength demo cafe error payload={slots}"
    );
    assert_eq!(
        slots["slot_count"].as_u64(),
        Some(35),
        "tool=get_venue_slot_strength expected 35 cells; payload={slots}"
    );
    assert_eq!(
        slots["slot_demand_profile"].as_array().map(|a| a.len()),
        Some(35),
        "tool=get_venue_slot_strength profile length; payload={slots}"
    );

    let slots_rejected = tool_payload(
        server.addr,
        "get_venue_slot_strength",
        json!({"path": "../outside.xlsx"}),
    );
    assert!(
        slots_rejected.get("error").is_some(),
        "tool=get_venue_slot_strength expected path-gate error; payload={slots_rejected}"
    );
    let slots_err = slots_rejected["error"].as_str().unwrap_or("");
    assert!(
        slots_err.contains("get_setup_status") || slots_err.contains("list_workspaces"),
        "path-gate error should suggest setup recovery; got {slots_err}"
    );

    let lift = tool_payload(
        server.addr,
        "get_pair_lift_matrix",
        json!({"path": "demo-cafe-july-2026"}),
    );
    assert!(
        lift.get("error").is_none(),
        "tool=get_pair_lift_matrix demo cafe error payload={lift}"
    );
    let focus = lift["focus_menus"].as_array().expect("focus_menus array");
    assert!(
        focus.len() >= 2,
        "tool=get_pair_lift_matrix expected >=2 focus menus; payload={lift}"
    );
    let matrix_lift = lift["matrix_lift"].as_array().expect("matrix_lift array");
    assert_eq!(
        matrix_lift.len(),
        focus.len(),
        "tool=get_pair_lift_matrix matrix rows must match focus; payload={lift}"
    );
    assert_eq!(
        lift["scope"].as_str(),
        Some("stars"),
        "tool=get_pair_lift_matrix demo cafe with COGS should use star focus; payload={lift}"
    );

    let lift_sales_only = tool_payload(
        server.addr,
        "get_pair_lift_matrix",
        json!({"path": "demo-cafe-july-2026/sales.xlsx"}),
    );
    assert!(
        lift_sales_only.get("error").is_none(),
        "tool=get_pair_lift_matrix sales Excel should succeed without COGS; payload={lift_sales_only}"
    );
    assert_eq!(
        lift_sales_only["scope"].as_str(),
        Some("top_by_presence"),
        "tool=get_pair_lift_matrix sales Excel path uses top_by_presence; payload={lift_sales_only}"
    );

    let rejected = tool_payload(
        server.addr,
        "get_pair_lift_matrix",
        json!({"path": "../outside.xlsx"}),
    );
    assert!(
        rejected.get("error").is_some(),
        "tool=get_pair_lift_matrix expected path-gate error; payload={rejected}"
    );
}

#[test]
fn mcp_http_reads_and_writes_workspace_knowledge() {
    let _guard = port_lock().lock().unwrap();
    let tmp = tempfile::tempdir().unwrap();
    fs::create_dir_all(tmp.path().join("cafe")).unwrap();
    fs::write(
        tmp.path().join("cafe/workspace.yaml"),
        r#"version: 1
name: Cafe
venue: Cafe
files:
  sales:
    path: sales.xlsx
"#,
    )
    .unwrap();

    let server = start_server(&tmp.path().to_path_buf());
    let missing = tool_payload(
        server.addr,
        "read_workspace_knowledge",
        json!({"path": "cafe"}),
    );
    assert_eq!(
        missing["exists"], false,
        "tool=read_workspace_knowledge missing payload={missing}"
    );
    assert_eq!(missing["knowledge_path"], "cafe/knowledge.md");

    let body = "# Cafe\n\nOpen daily 09:00–21:00.\n";
    let written = tool_payload(
        server.addr,
        "write_workspace_knowledge",
        json!({"path": "cafe", "content": body}),
    );
    assert_eq!(
        written["knowledge_path"], "cafe/knowledge.md",
        "tool=write_workspace_knowledge payload={written}"
    );
    assert_eq!(written["config_updated"], true);
    assert_eq!(written["content"], body);
    assert!(tmp.path().join("cafe/knowledge.md").is_file());

    let read = tool_payload(
        server.addr,
        "read_workspace_knowledge",
        json!({"path": "cafe"}),
    );
    assert_eq!(read["exists"], true);
    assert_eq!(read["content"], body);

    let rejected = tool_payload(
        server.addr,
        "write_workspace_knowledge",
        json!({"path": "../outside", "content": "nope"}),
    );
    assert!(
        rejected.get("error").is_some(),
        "expected outside-sandbox error, got {rejected}"
    );
    let read_rejected = tool_payload(
        server.addr,
        "read_workspace_knowledge",
        json!({"path": "../outside"}),
    );
    assert!(
        read_rejected.get("error").is_some(),
        "expected outside-sandbox read error, got {read_rejected}"
    );
}

#[test]
fn mcp_http_writes_workspace_config() {
    let _guard = port_lock().lock().unwrap();
    let tmp = tempfile::tempdir().unwrap();
    fs::create_dir_all(tmp.path().join("new-cafe")).unwrap();
    fs::write(tmp.path().join("new-cafe/sales.xlsx"), b"x").unwrap();
    fs::write(tmp.path().join("new-cafe/cogs.xlsx"), b"y").unwrap();

    let server = start_server(&tmp.path().to_path_buf());
    let written = tool_payload(
        server.addr,
        "write_workspace_config",
        json!({
            "path": "new-cafe",
            "config": {
                "version": 1,
                "name": "New Cafe — Test",
                "venue": "New Cafe",
                "currency": "IDR",
                "files": {
                    "sales": {"path": "sales.xlsx", "description": "POS export"},
                    "cogs": {"path": "cogs.xlsx", "description": "COGS"}
                }
            }
        }),
    );
    assert_eq!(
        written["config_path"], "new-cafe/workspace.yaml",
        "tool=write_workspace_config payload={written}"
    );
    assert!(
        tmp.path().join("new-cafe/workspace.yaml").is_file(),
        "workspace.yaml was not written"
    );

    let rejected = tool_payload(
        server.addr,
        "write_workspace_config",
        json!({
            "path": "../outside",
            "config": {
                "version": 1,
                "name": "Nope",
                "venue": "Nope",
                "files": {"sales": {"path": "sales.xlsx"}}
            }
        }),
    );
    assert!(
        rejected.get("error").is_some(),
        "expected outside-sandbox error, got {rejected}"
    );
}

#[test]
fn mcp_http_hello_world_widget_resource_and_structured_content() {
    let _guard = port_lock().lock().unwrap();
    let data_dir = repo_root().join("workspaces");
    let server = start_server(&data_dir);

    let call = http_post_json(
        server.addr,
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": { "name": "render_hello_world_widget", "arguments": {} }
        }),
    );
    let result = &call["result"];
    assert_eq!(
        result["structuredContent"]["message"].as_str(),
        Some("Hello World"),
        "expected mock structuredContent.message; call={call}"
    );
    assert_eq!(
        result["structuredContent"]["source"].as_str(),
        Some("mock"),
        "expected mock structuredContent.source; call={call}"
    );
    assert_eq!(
        result["_meta"]["ui"]["resourceUri"].as_str(),
        Some("ui://widget/hello-world-v3.html"),
        "tool result must include _meta.ui.resourceUri; call={call}"
    );
    assert_eq!(
        result["_meta"]["openai/outputTemplate"].as_str(),
        Some("ui://widget/hello-world-v3.html"),
        "tool result must include openai/outputTemplate; call={call}"
    );
    let text = result["content"][0]["text"]
        .as_str()
        .unwrap_or_else(|| panic!("missing text content; call={call}"));
    assert!(
        text.contains("Hello World"),
        "content text should include Hello World; text={text}"
    );

    let listed = http_post_json(
        server.addr,
        json!({"jsonrpc":"2.0","id":2,"method":"resources/list","params":{}}),
    );
    let resources = listed["result"]["resources"]
        .as_array()
        .unwrap_or_else(|| panic!("resources array missing; listed={listed}"));
    let hello = resources
        .iter()
        .find(|r| r["uri"] == "ui://widget/hello-world-v3.html")
        .unwrap_or_else(|| panic!("hello-world resource missing; listed={listed}"));
    assert_eq!(
        hello["mimeType"].as_str(),
        Some("text/html+skybridge"),
        "resources/list mimeType; resource={hello}"
    );

    let read = http_post_json(
        server.addr,
        json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "resources/read",
            "params": { "uri": "ui://widget/hello-world-v3.html" }
        }),
    );
    let contents = read["result"]["contents"]
        .as_array()
        .unwrap_or_else(|| panic!("contents array missing; read={read}"));
    assert_eq!(contents.len(), 1, "expected one content block; read={read}");
    assert_eq!(
        contents[0]["uri"].as_str(),
        Some("ui://widget/hello-world-v3.html"),
        "resources/read uri; read={read}"
    );
    assert_eq!(
        contents[0]["mimeType"].as_str(),
        Some("text/html+skybridge"),
        "resources/read mimeType; read={read}"
    );
    let html = contents[0]["text"]
        .as_str()
        .unwrap_or_else(|| panic!("missing HTML text; read={read}"));
    assert!(
        html.contains("Hello World") || html.contains("id=\"label\""),
        "widget HTML should render a Hello World label; html_len={}",
        html.len()
    );
    assert!(
        html.contains("structuredContent") || html.contains("tool-result"),
        "widget HTML should read mock structuredContent from tool-result; html_len={}",
        html.len()
    );
}

#[test]
fn mcp_http_venue_slot_strength_widget_render_and_resource() {
    let _guard = port_lock().lock().unwrap();
    let data_dir = repo_root().join("workspaces");
    let server = start_server(&data_dir);

    let call = http_post_json(
        server.addr,
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": "render_venue_slot_strength",
                "arguments": { "path": "demo-cafe-july-2026" }
            }
        }),
    );
    let result = &call["result"];
    assert!(
        result.get("isError").and_then(|v| v.as_bool()) != Some(true),
        "render_venue_slot_strength error; call={call}"
    );
    assert_eq!(
        result["structuredContent"]["slot_count"].as_u64(),
        Some(35),
        "expected 35-cell structuredContent; call={call}"
    );
    assert_eq!(
        result["structuredContent"]["slot_demand_profile"]
            .as_array()
            .map(|a| a.len()),
        Some(35),
        "expected 35-cell profile; call={call}"
    );
    assert_eq!(
        result["_meta"]["ui"]["resourceUri"].as_str(),
        Some("ui://widget/venue-slot-strength-v6.html"),
        "tool result must include template URI; call={call}"
    );
    assert_eq!(
        result["_meta"]["openai/outputTemplate"].as_str(),
        Some("ui://widget/venue-slot-strength-v6.html"),
        "tool result must include openai/outputTemplate; call={call}"
    );
    assert_eq!(
        result["_meta"]["ui"]["prefersBorder"].as_bool(),
        Some(true),
        "tool result should prefer host border/padding; call={call}"
    );
    assert_eq!(
        result["_meta"]["openai/widgetPrefersBorder"].as_bool(),
        Some(true),
        "tool result missing openai/widgetPrefersBorder; call={call}"
    );
    let text = result["content"][0]["text"]
        .as_str()
        .unwrap_or_else(|| panic!("missing text content; call={call}"));
    assert!(
        text.to_lowercase().contains("venue slot"),
        "content text should mention venue slot; text={text}"
    );

    let listed = http_post_json(
        server.addr,
        json!({"jsonrpc":"2.0","id":2,"method":"resources/list","params":{}}),
    );
    let resources = listed["result"]["resources"]
        .as_array()
        .unwrap_or_else(|| panic!("resources array missing; listed={listed}"));
    let venue = resources
        .iter()
        .find(|r| r["uri"] == "ui://widget/venue-slot-strength-v6.html")
        .unwrap_or_else(|| panic!("venue slot resource missing; listed={listed}"));
    assert_eq!(
        venue["mimeType"].as_str(),
        Some("text/html+skybridge"),
        "resources/list mimeType; resource={venue}"
    );

    let read = http_post_json(
        server.addr,
        json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "resources/read",
            "params": { "uri": "ui://widget/venue-slot-strength-v6.html" }
        }),
    );
    let contents = read["result"]["contents"]
        .as_array()
        .unwrap_or_else(|| panic!("contents array missing; read={read}"));
    assert_eq!(contents.len(), 1, "expected one content block; read={read}");
    assert_eq!(
        contents[0]["mimeType"].as_str(),
        Some("text/html+skybridge"),
        "resources/read mimeType; read={read}"
    );
    let html = contents[0]["text"]
        .as_str()
        .unwrap_or_else(|| panic!("missing HTML text; read={read}"));
    assert!(
        html.contains("slot_demand_profile") || html.contains("Venue slot strength"),
        "widget HTML should render venue slot strength; html_len={}",
        html.len()
    );
    assert!(
        html.contains("--pad-top: 48px"),
        "widget HTML should keep extra top padding for ChatGPT/Codex; html_len={}",
        html.len()
    );
    assert!(
        html.contains("color-scheme: light dark") || html.contains("applyDocumentTheme"),
        "widget HTML should theme for host light/dark; html_len={}",
        html.len()
    );
    assert!(
        html.contains("--color-background-primary") && html.contains("host-context-changed"),
        "widget HTML should apply host style tokens / context changes; html_len={}",
        html.len()
    );
    assert!(
        html.contains("metric-seg")
            && html.contains("data-metric=\"demand\"")
            && html.contains("data-metric=\"orders\"")
            && html.contains("data-metric=\"share\""),
        "widget HTML should offer Demand/Orders/Share metric selector; html_len={}",
        html.len()
    );
    assert!(
        html.contains("menuyukti-venue-slot-metric"),
        "widget HTML should persist metric selection; html_len={}",
        html.len()
    );
}

#[test]
fn mcp_http_pair_lift_matrix_widget_render_and_resource() {
    let _guard = port_lock().lock().unwrap();
    let data_dir = repo_root().join("workspaces");
    let server = start_server(&data_dir);

    let call = http_post_json(
        server.addr,
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": "render_pair_lift_matrix",
                "arguments": { "path": "demo-cafe-july-2026" }
            }
        }),
    );
    let result = &call["result"];
    assert!(
        result.get("isError").and_then(|v| v.as_bool()) != Some(true),
        "render_pair_lift_matrix error; call={call}"
    );
    let focus = result["structuredContent"]["focus_menus"]
        .as_array()
        .unwrap_or_else(|| panic!("focus_menus missing; call={call}"));
    assert!(focus.len() >= 2, "expected >=2 focus menus; call={call}");
    let matrix = result["structuredContent"]["matrix_lift"]
        .as_array()
        .unwrap_or_else(|| panic!("matrix_lift missing; call={call}"));
    assert_eq!(
        matrix.len(),
        focus.len(),
        "matrix rows must match focus; call={call}"
    );
    assert!(
        matrix[0]
            .as_array()
            .and_then(|r| r.first())
            .is_some_and(|c| c.is_null()),
        "diagonal should be null; call={call}"
    );
    assert_eq!(
        result["_meta"]["ui"]["resourceUri"].as_str(),
        Some("ui://widget/pair-lift-matrix-v5.html"),
        "tool result must include template URI; call={call}"
    );
    assert_eq!(
        result["_meta"]["openai/outputTemplate"].as_str(),
        Some("ui://widget/pair-lift-matrix-v5.html"),
        "tool result must include openai/outputTemplate; call={call}"
    );
    assert_eq!(
        result["_meta"]["ui"]["prefersBorder"].as_bool(),
        Some(true),
        "tool result should prefer host border/padding; call={call}"
    );
    assert_eq!(
        result["_meta"]["openai/widgetPrefersBorder"].as_bool(),
        Some(true),
        "tool result missing openai/widgetPrefersBorder; call={call}"
    );
    let text = result["content"][0]["text"]
        .as_str()
        .unwrap_or_else(|| panic!("missing text content; call={call}"));
    assert!(
        text.to_lowercase().contains("pair lift"),
        "content text should mention pair lift; text={text}"
    );

    let listed = http_post_json(
        server.addr,
        json!({"jsonrpc":"2.0","id":2,"method":"resources/list","params":{}}),
    );
    let resources = listed["result"]["resources"]
        .as_array()
        .unwrap_or_else(|| panic!("resources array missing; listed={listed}"));
    let pair = resources
        .iter()
        .find(|r| r["uri"] == "ui://widget/pair-lift-matrix-v5.html")
        .unwrap_or_else(|| panic!("pair lift resource missing; listed={listed}"));
    assert_eq!(
        pair["mimeType"].as_str(),
        Some("text/html+skybridge"),
        "resources/list mimeType; resource={pair}"
    );

    let read = http_post_json(
        server.addr,
        json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "resources/read",
            "params": { "uri": "ui://widget/pair-lift-matrix-v5.html" }
        }),
    );
    let contents = read["result"]["contents"]
        .as_array()
        .unwrap_or_else(|| panic!("contents array missing; read={read}"));
    assert_eq!(contents.len(), 1, "expected one content block; read={read}");
    assert_eq!(
        contents[0]["mimeType"].as_str(),
        Some("text/html+skybridge"),
        "resources/read mimeType; read={read}"
    );
    let html = contents[0]["text"]
        .as_str()
        .unwrap_or_else(|| panic!("missing HTML text; read={read}"));
    assert!(
        html.contains("matrix_lift") || html.contains("Pair lift index"),
        "widget HTML should render pair lift; html_len={}",
        html.len()
    );
    assert!(
        html.contains("--pad-top: 48px"),
        "widget HTML should keep extra top padding for ChatGPT/Codex; html_len={}",
        html.len()
    );
    assert!(
        html.contains("color-scheme: light dark") || html.contains("applyDocumentTheme"),
        "widget HTML should theme for host light/dark; html_len={}",
        html.len()
    );
    assert!(
        html.contains("--color-background-primary") && html.contains("host-context-changed"),
        "widget HTML should apply host style tokens / context changes; html_len={}",
        html.len()
    );
    assert!(
        html.contains("null-cell") && html.contains("Same item / no lift"),
        "widget HTML should distinguish null/diagonal cells; html_len={}",
        html.len()
    );
    assert!(
        html.contains("th.sortable") && html.contains("aria-sort") && html.contains("toggleSort"),
        "widget HTML should support sortable column headers; html_len={}",
        html.len()
    );
}

#[test]
fn mcp_http_menu_heatmaps_widget_render_and_resource() {
    let _guard = port_lock().lock().unwrap();
    let data_dir = repo_root().join("workspaces");
    let server = start_server(&data_dir);

    let call = http_post_json(
        server.addr,
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": "render_menu_heatmaps",
                "arguments": { "path": "demo-cafe-july-2026" }
            }
        }),
    );
    let result = &call["result"];
    assert!(
        result.get("isError").and_then(|v| v.as_bool()) != Some(true),
        "render_menu_heatmaps error; call={call}"
    );
    let heatmaps = result["structuredContent"]["heatmaps"]
        .as_array()
        .unwrap_or_else(|| panic!("heatmaps missing; call={call}"));
    assert!(!heatmaps.is_empty(), "expected heatmap items; call={call}");
    assert!(
        heatmaps
            .iter()
            .any(|item| item["matrix_category"].as_str().is_some()),
        "demo cafe render should enrich matrix_category; call={call}"
    );
    assert_eq!(
        result["_meta"]["ui"]["resourceUri"].as_str(),
        Some("ui://widget/menu-heatmaps-v2.html"),
        "tool result must include template URI; call={call}"
    );
    assert_eq!(
        result["_meta"]["openai/outputTemplate"].as_str(),
        Some("ui://widget/menu-heatmaps-v2.html"),
        "tool result must include openai/outputTemplate; call={call}"
    );
    assert_eq!(
        result["_meta"]["ui"]["prefersBorder"].as_bool(),
        Some(true),
        "tool result should prefer host border/padding; call={call}"
    );
    let text = result["content"][0]["text"]
        .as_str()
        .unwrap_or_else(|| panic!("missing text content; call={call}"));
    assert!(
        text.to_lowercase().contains("menu heatmap"),
        "content text should mention menu heatmaps; text={text}"
    );

    let listed = http_post_json(
        server.addr,
        json!({"jsonrpc":"2.0","id":2,"method":"resources/list","params":{}}),
    );
    let resources = listed["result"]["resources"]
        .as_array()
        .unwrap_or_else(|| panic!("resources array missing; listed={listed}"));
    let menus = resources
        .iter()
        .find(|r| r["uri"] == "ui://widget/menu-heatmaps-v2.html")
        .unwrap_or_else(|| panic!("menu heatmaps resource missing; listed={listed}"));
    assert_eq!(
        menus["mimeType"].as_str(),
        Some("text/html+skybridge"),
        "resources/list mimeType; resource={menus}"
    );

    let read = http_post_json(
        server.addr,
        json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "resources/read",
            "params": { "uri": "ui://widget/menu-heatmaps-v2.html" }
        }),
    );
    let contents = read["result"]["contents"]
        .as_array()
        .unwrap_or_else(|| panic!("contents array missing; read={read}"));
    assert_eq!(contents.len(), 1, "expected one content block; read={read}");
    assert_eq!(
        contents[0]["mimeType"].as_str(),
        Some("text/html+skybridge"),
        "resources/read mimeType; read={read}"
    );
    let html = contents[0]["text"]
        .as_str()
        .unwrap_or_else(|| panic!("missing HTML text; read={read}"));
    assert!(
        html.contains("Menu heatmaps") && html.contains("heatmaps"),
        "widget HTML should render menu heatmaps; html_len={}",
        html.len()
    );
    assert!(
        html.contains("--pad-top: 48px"),
        "widget HTML should keep extra top padding for ChatGPT/Codex; html_len={}",
        html.len()
    );
    assert!(
        html.contains("data-category=\"weak\"") && html.contains("data-category=\"star\""),
        "widget HTML should offer matrix category filters; html_len={}",
        html.len()
    );
    assert!(
        html.contains("data-view=\"weekly\"") && html.contains("data-view=\"daily\""),
        "widget HTML should offer daily/weekly view toggle; html_len={}",
        html.len()
    );
    assert!(
        html.contains("totals-row") && html.contains("peak-total"),
        "widget HTML should include totals row with peak highlight; html_len={}",
        html.len()
    );
    assert!(
        html.contains("th.sortable") && html.contains("toggleSort"),
        "widget HTML should support sortable columns; html_len={}",
        html.len()
    );
}

#[test]
fn mcp_http_menu_engineering_requires_cogs() {
    let _guard = port_lock().lock().unwrap();
    let tmp = tempfile::tempdir().unwrap();
    fs::create_dir_all(tmp.path().join("sales-only")).unwrap();
    // Minimal valid workspace with sales role only (no COGS).
    fs::write(
        tmp.path().join("sales-only/workspace.yaml"),
        r#"version: 1
name: Sales Only
venue: Sales Only
files:
  sales:
    path: sales.xlsx
"#,
    )
    .unwrap();
    fs::write(tmp.path().join("sales-only/sales.xlsx"), b"x").unwrap();

    let server = start_server(&tmp.path().to_path_buf());
    let missing = tool_payload(
        server.addr,
        "analyze_menu_engineering",
        json!({"path": "sales-only"}),
    );
    assert!(
        missing.get("error").is_some(),
        "expected missing-COGS error, got {missing}"
    );
    let err = missing["error"].as_str().unwrap_or("");
    assert!(
        err.contains("cogs") || err.contains("COGS"),
        "error should mention COGS; got {err}"
    );
    assert!(
        err.contains("list_workspace_files") && err.contains("write_workspace_config"),
        "COGS error should include recovery tools; got {err}"
    );

    let weak_missing = tool_payload(
        server.addr,
        "get_weak_items_review",
        json!({"path": "sales-only"}),
    );
    assert!(
        weak_missing.get("error").is_some(),
        "expected weak-items missing-COGS error, got {weak_missing}"
    );
    let weak_err = weak_missing["error"].as_str().unwrap_or("");
    assert!(
        weak_err.contains("cogs") || weak_err.contains("COGS"),
        "weak-items error should mention COGS; got {weak_err}"
    );
    assert!(
        weak_err.contains("list_workspace_files") && weak_err.contains("write_workspace_config"),
        "weak-items COGS error should include recovery tools; got {weak_err}"
    );
}

#[test]
fn mcp_http_setup_status_and_owner_playbook_instructions() {
    let _guard = port_lock().lock().unwrap();
    let tmp = tempfile::tempdir().unwrap();
    fs::create_dir_all(tmp.path().join("needs-setup")).unwrap();
    fs::write(tmp.path().join("needs-setup/sales.xlsx"), b"x").unwrap();
    fs::create_dir_all(tmp.path().join("demo-cafe-july-2026")).unwrap();
    fs::write(
        tmp.path().join("demo-cafe-july-2026/workspace.yaml"),
        "version: 1\nname: Demo\nvenue: Demo\nfiles:\n  sales:\n    path: sales.xlsx\n",
    )
    .unwrap();
    fs::write(tmp.path().join("demo-cafe-july-2026/sales.xlsx"), b"y").unwrap();

    let server = start_server(&tmp.path().to_path_buf());

    let init = http_post_json(
        server.addr,
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": { "name": "menuyukti-test", "version": "0" }
            }
        }),
    );
    let instructions = init["result"]["instructions"]
        .as_str()
        .unwrap_or_else(|| panic!("initialize missing instructions; init={init}"));
    for needle in [
        "get_setup_status",
        "demo-cafe-july-2026",
        "write_workspace_config",
        "render_venue_slot_strength",
        "render_pair_lift_matrix",
        "render_menu_heatmaps",
        "list_workspace_files",
    ] {
        assert!(
            instructions.contains(needle),
            "instructions missing `{needle}`; instructions={instructions}"
        );
    }

    let status = tool_payload(server.addr, "get_setup_status", json!({}));
    assert_eq!(
        status["server"].as_str(),
        Some("menuyukti-mcp"),
        "setup status server; payload={status}"
    );
    assert!(
        status["version"].as_str().is_some(),
        "setup status version; payload={status}"
    );
    assert_eq!(
        status["demo_workspace_present"].as_bool(),
        Some(true),
        "demo should be present; payload={status}"
    );
    assert_eq!(
        status["workspace_count"].as_u64(),
        Some(2),
        "expected two workspace folders; payload={status}"
    );
    let missing = status["folders_missing_workspace_yaml"]
        .as_array()
        .expect("folders_missing_workspace_yaml array");
    assert!(
        missing.iter().any(|v| v.as_str() == Some("needs-setup")),
        "needs-setup should lack workspace.yaml; payload={status}"
    );

    let no_config = tool_payload(
        server.addr,
        "summarize_local_sales",
        json!({"path": "needs-setup"}),
    );
    assert!(
        no_config.get("error").is_some(),
        "expected missing workspace.yaml error; payload={no_config}"
    );
    let no_config_err = no_config["error"].as_str().unwrap_or("");
    assert!(
        no_config_err.contains("list_workspace_files")
            && no_config_err.contains("write_workspace_config"),
        "missing-config error should include recovery; got {no_config_err}"
    );
}
