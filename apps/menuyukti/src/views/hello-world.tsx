import { useToolInfo } from "../helpers.js";
import { Loading, Panel } from "./shared.js";

export default function HelloWorld() {
  const { output, isPending } = useToolInfo<"render_hello_world_widget">();

  if (isPending || !output) {
    return <Loading label="Hello World" />;
  }

  return (
    <Panel title={String(output.title ?? "Hello World")} description="">
      <p className="myk-meta" style={{ marginTop: 0 }}>
        {String(output.message ?? "")}
      </p>
      <p className="myk-meta">source: {String(output.source ?? "")}</p>
    </Panel>
  );
}
