/** JSON tool result helpers matching the Rust MCP error/success shape. */

export function okJson(value: unknown) {
  return {
    structuredContent: value as Record<string, unknown>,
    content: [{ type: "text" as const, text: JSON.stringify(value, null, 2) }],
    isError: false,
  };
}

/** Widget tool result: short host text + structuredContent for the Skybridge view. */
export function widgetJson(value: unknown, message: string) {
  return {
    structuredContent: value as Record<string, unknown>,
    content: [{ type: "text" as const, text: message }],
    isError: false,
  };
}

export function errJson(message: string) {
  const value = { error: message };
  return {
    structuredContent: value,
    content: [{ type: "text" as const, text: JSON.stringify(value) }],
    isError: true,
  };
}
