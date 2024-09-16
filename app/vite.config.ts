import { defineConfig } from "vite";
import top_level_await from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";

export default defineConfig({
  server: { port: 3000 },
  plugins: [wasm(), top_level_await()],
});
