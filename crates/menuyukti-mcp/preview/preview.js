/**
 * Widget preview harness: inject mock structuredContent into the iframe
 * the same way ChatGPT MCP Apps does, and reload when the preview server
 * bumps its watch version.
 *
 * Optional theme toggle (`#theme-toggle` or `data-theme` on body) posts a
 * mock host-context-changed notification so widgets can be checked in dark mode.
 */
(function () {
  const body = document.body;
  const widgetSrc = body.dataset.widget;
  const fixtureSrc = body.dataset.fixture;
  const frame = document.getElementById("widget-frame");
  const statusEl = document.getElementById("status");
  const themeToggle = document.getElementById("theme-toggle");
  const THEME_KEY = "menuyukti-widget-preview-theme";

  function setStatus(text) {
    if (statusEl) statusEl.textContent = text;
  }

  function currentTheme() {
    const fromBody = body.dataset.theme;
    if (fromBody === "dark" || fromBody === "light") return fromBody;
    try {
      const stored = localStorage.getItem(THEME_KEY);
      if (stored === "dark" || stored === "light") return stored;
    } catch (_) {
      /* ignore */
    }
    return "light";
  }

  function setTheme(theme) {
    const mode = theme === "dark" ? "dark" : "light";
    body.dataset.theme = mode;
    try {
      localStorage.setItem(THEME_KEY, mode);
    } catch (_) {
      /* ignore */
    }
    if (themeToggle) {
      themeToggle.textContent = mode === "dark" ? "Light mode" : "Dark mode";
      themeToggle.setAttribute("aria-pressed", mode === "dark" ? "true" : "false");
    }
    if (frame) {
      frame.style.background = mode === "dark" ? "#171717" : "#ffffff";
    }
    postHostContext(mode);
  }

  function postHostContext(theme) {
    if (!frame?.contentWindow) return;
    const hostContext = {
      theme: theme || currentTheme(),
      styles: {
        variables: {},
      },
    };
    frame.contentWindow.postMessage(
      {
        jsonrpc: "2.0",
        method: "ui/notifications/host-context-changed",
        params: { hostContext },
      },
      "*"
    );
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
      postHostContext(currentTheme());
      const structuredContent = await loadFixture();
      postToolResult(structuredContent);
      setStatus(`Mock injected from ${fixtureSrc} · theme ${currentTheme()}`);
    } catch (err) {
      setStatus(`Mock inject failed: ${String(err)}`);
    }
  }

  // Answer ui/initialize the way a host would (theme in hostContext).
  window.addEventListener("message", (event) => {
    if (event.source !== frame.contentWindow) return;
    const message = event.data;
    if (!message || message.jsonrpc !== "2.0") return;
    if (message.method === "ui/initialize" && message.id != null) {
      frame.contentWindow.postMessage(
        {
          jsonrpc: "2.0",
          id: message.id,
          result: {
            protocolVersion: "2026-01-26",
            hostContext: { theme: currentTheme() },
          },
        },
        "*"
      );
    }
  });

  if (themeToggle) {
    themeToggle.addEventListener("click", () => {
      setTheme(currentTheme() === "dark" ? "light" : "dark");
      setStatus(`Theme → ${currentTheme()}`);
    });
  }

  setTheme(currentTheme());
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
