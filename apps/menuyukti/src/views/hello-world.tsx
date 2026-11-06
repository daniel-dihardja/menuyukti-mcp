import type { CSSProperties } from "react";
import { useToolInfo } from "../helpers.js";

export default function HelloWorld() {
  const { output, isPending } = useToolInfo<"render_hello_world_widget">();

  if (isPending || !output) {
    return (
      <main style={shell}>
        <p style={muted}>Loading Hello World…</p>
      </main>
    );
  }

  return (
    <main style={shell}>
      <h1 style={title}>{output.title}</h1>
      <p style={body}>{output.message}</p>
      <p style={muted}>source: {output.source}</p>
    </main>
  );
}

const shell: CSSProperties = {
  fontFamily:
    'ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif',
  padding: "16px 18px",
  margin: 0,
  color: "#111827",
  background: "#f8fafc",
};

const title: CSSProperties = {
  margin: "0 0 8px",
  fontSize: "1.25rem",
  fontWeight: 700,
};

const body: CSSProperties = {
  margin: "0 0 8px",
  fontSize: "0.95rem",
  lineHeight: 1.45,
};

const muted: CSSProperties = {
  margin: 0,
  fontSize: "0.8rem",
  color: "#64748b",
};
