import path from "node:path";
import { fileURLToPath } from "node:url";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

const root = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  root: path.join(root, "preview"),
  plugins: [react()],
  resolve: {
    alias: [
      {
        // Views import "../helpers.js"; serve fixture mock instead of Skybridge bridge.
        find: /(?:^|\/)helpers\.js$/,
        replacement: path.join(root, "preview/mock-helpers.ts"),
      },
      {
        find: "@",
        replacement: path.join(root, "src"),
      },
    ],
  },
  server: {
    port: 8790,
    strictPort: true,
  },
});
