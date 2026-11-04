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
    ] {
        assert!(
            names.contains(&required),
            "tools/list missing {required}; got {names:?}"
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
}
