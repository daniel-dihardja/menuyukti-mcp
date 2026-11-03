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

    let bin = env!("CARGO_BIN_EXE_gastro-mcp");
    let mut child = Command::new(bin)
        .env("MENUYUKTI_DATA_DIR", data_dir)
        .env("MENUYUKTI_MCP_PORT", port.to_string())
        .env("RUST_LOG", "error")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn gastro-mcp");

    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        if TcpStream::connect_timeout(&addr, Duration::from_millis(100)).is_ok() {
            return RunningServer { child, addr };
        }
        thread::sleep(Duration::from_millis(50));
    }
    let _ = child.kill();
    let _ = child.wait();
    panic!("gastro-mcp failed to listen on {addr}");
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
        "summarize_local_sales",
        "list_normalized_sales",
        "get_menu_heatmaps",
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
