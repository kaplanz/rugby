import { execSync } from "node:child_process";

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
  plugins: [wasm()],
  publicDir: "www",
});
