/**
 * Widget preview harness: inject mock structuredContent into the iframe
 * the same way ChatGPT MCP Apps does, and reload when the preview server
 * bumps its watch version.
 */
(function () {
  const body = document.body;
  const widgetSrc = body.dataset.widget;
  const fixtureSrc = body.dataset.fixture;
  const frame = document.getElementById("widget-frame");
  const statusEl = document.getElementById("status");

  function setStatus(text) {
    if (statusEl) statusEl.textContent = text;
  }

  async function loadFixture() {
    const res = await fetch(fixtureSrc, { cache: "no-store" });
    if (!res.ok) {
      throw new Error(`fixture ${fixtureSrc}: HTTP ${res.status}`);
    }
    return res.json();
  }

  function postToolResult(structuredContent) {
    const message = {
      jsonrpc: "2.0",
      method: "ui/notifications/tool-result",
      params: { structuredContent },
    };
    frame.contentWindow.postMessage(message, "*");
  }

  async function injectMock() {
    try {
      const structuredContent = await loadFixture();
      postToolResult(structuredContent);
      setStatus(`Mock injected from ${fixtureSrc}`);
    } catch (err) {
      setStatus(`Mock inject failed: ${String(err)}`);
    }
  }

  frame.src = widgetSrc;
  frame.addEventListener("load", () => {
    // Give the widget module a tick to attach its message listener.
    setTimeout(injectMock, 0);
  });

  // Live reload via SSE (falls back to version polling).
  let watchVersion = null;
  try {
    const source = new EventSource("/__reload");
    source.addEventListener("reload", () => {
      location.reload();
    });
    source.onerror = () => {
      /* EventSource reconnects; polling covers hard failures */
    };
  } catch (_) {
    /* ignore */
  }

  async function pollWatch() {
    try {
      const res = await fetch("/__watch", { cache: "no-store" });
      if (!res.ok) return;
      const data = await res.json();
      if (watchVersion === null) {
        watchVersion = data.version;
      } else if (data.version !== watchVersion) {
        location.reload();
      }
    } catch (_) {
      /* server may restart briefly */
    }
  }
  setInterval(pollWatch, 500);
})();
