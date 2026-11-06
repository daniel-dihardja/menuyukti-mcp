/**
 * Widget preview harness: inject mock structuredContent into the iframe
 * the same way ChatGPT MCP Apps does.
 *
 * No live reload — refresh the browser after editing widget HTML or fixtures.
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

  if (!widgetSrc || !fixtureSrc || !frame) {
    setStatus("Missing data-widget / data-fixture / #widget-frame");
    return;
  }

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
    frame.style.background = mode === "dark" ? "#171717" : "#ffffff";
    postHostContext(mode);
  }

  function postHostContext(theme) {
    if (!frame.contentWindow) return;
    frame.contentWindow.postMessage(
      {
        jsonrpc: "2.0",
        method: "ui/notifications/host-context-changed",
        params: {
          hostContext: {
            theme: theme || currentTheme(),
            styles: { variables: {} },
          },
        },
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
    frame.contentWindow.postMessage(
      {
        jsonrpc: "2.0",
        method: "ui/notifications/tool-result",
        params: { structuredContent },
      },
      "*"
    );
  }

  let cachedStructured = null;
  let injectGeneration = 0;

  async function injectMock(reason) {
    const generation = ++injectGeneration;
    try {
      postHostContext(currentTheme());
      if (!cachedStructured) {
        cachedStructured = await loadFixture();
      }
      if (generation !== injectGeneration) return;
      postToolResult(cachedStructured);
      setStatus(
        `Mock injected from ${fixtureSrc} · theme ${currentTheme()}` +
          (reason ? ` · ${reason}` : "")
      );
    } catch (err) {
      setStatus(`Mock inject failed: ${String(err)}`);
    }
  }

  // Handshake + inject once the widget has attached its listener.
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
      injectMock("handshake");
    }
  });

  if (themeToggle) {
    themeToggle.addEventListener("click", () => {
      setTheme(currentTheme() === "dark" ? "light" : "dark");
      if (cachedStructured) {
        postToolResult(cachedStructured);
      }
      setStatus(`Theme → ${currentTheme()}`);
    });
  }

  setTheme(currentTheme());
  frame.src = widgetSrc;
  frame.addEventListener("load", () => {
    cachedStructured = null;
    setTimeout(() => injectMock("load"), 0);
    setTimeout(() => injectMock("retry"), 100);
  });
})();
