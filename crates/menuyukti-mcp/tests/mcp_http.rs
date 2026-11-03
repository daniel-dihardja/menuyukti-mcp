use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::{json, Value};

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
    // The binary hardcodes 8787; pick a free port by temporarily binding then
    // requiring the default only when free. If 8787 is busy, skip.
    let listener = TcpListener::bind("127.0.0.1:8787");
    let addr = match listener {
        Ok(l) => {
            let addr = l.local_addr().unwrap();
            drop(l);
            addr
        }
        Err(_) => {
            // Fall back: still try default; test will fail clearly if busy.
            "127.0.0.1:8787".parse().unwrap()
        }
    };
    let _ = free_port(); // keep helper used for future flexible bind

    let bin = env!("CARGO_BIN_EXE_menuyukti-mcp");
    let mut child = Command::new(bin)
        .env("MENUYUKTI_DATA_DIR", data_dir)
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
}
