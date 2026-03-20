import { execSync } from "node:child_process";
import { fileURLToPath } from "node:url";

import { defineConfig } from "vite";

import wasm from "vite-plugin-wasm";

// https://vitejs.dev/config/
export default defineConfig({
  define: {
    BUILD: {
      COMMIT: execSync("git rev-parse --short HEAD").toString().trim(),
    },
  },
  esbuild: {
    supported: {
      "top-level-await": true,
    },
  },
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
  plugins: [wasm()],
  build: {
    lib: {
      entry: "src/lib.ts",
      formats: ["es"],
      fileName: "rugby",
    },
  },
});
