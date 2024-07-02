import { svelte } from "@sveltejs/vite-plugin-svelte";
import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";

// https://vitejs.dev/config/
export default defineConfig({
  esbuild: {
    supported: {
      "top-level-await": true,
    },
  },
  plugins: [svelte(), wasm()],
});
