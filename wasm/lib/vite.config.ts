import { defineConfig } from "vite";
import path from "path";
import wasm from "vite-plugin-wasm";

export default defineConfig({
  plugins: [wasm()],
  build: {
    lib: {
      entry: path.resolve(__dirname, "wasm/index.js"),
      name: "payment_plan",
      fileName: "index",
      formats: ["es"],
    },
    minify: false,
    target: "esnext",
    outDir: "dist",
  },
});
