/** Vite preview alias for src/helpers — feeds fixture structuredContent into views. */

declare global {
  interface Window {
    __MENUYUKTI_PREVIEW__?: {
      output: unknown;
      isPending?: boolean;
    };
  }
}

export function useToolInfo(_name?: string) {
  const state = window.__MENUYUKTI_PREVIEW__ ?? {
    output: null,
    isPending: true,
  };
  return {
    output: state.output as never,
    isPending: state.isPending ?? false,
    toolInput: null,
  };
}

export function useCallTool() {
  return async () => {
    throw new Error("useCallTool is unavailable in widget fixture preview");
  };
}
