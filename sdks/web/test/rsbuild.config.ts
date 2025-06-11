import { resolve } from "node:path";

import { defineConfig } from "@rsbuild/core";
import { pluginReact } from "@rsbuild/plugin-react";
import { pluginTypeCheck } from "@rsbuild/plugin-type-check";

export default defineConfig({
  plugins: [
    pluginReact(),
    pluginTypeCheck({
      enable: true,
      tsCheckerOptions: {
        typescript: {
          mode: "write-references",
        },
      },
    }),
  ],
  html: {
    template: "./index.html",
  },
  source: {
    entry: {
      index: "./src/main.tsx",
    },
  },
  resolve: {
    alias: {
      "@assets": resolve(__dirname, "./src/assets"),
      "@common": resolve(__dirname, "./src/common"),
      "@config": resolve(__dirname, "./src/config"),
      "@pages": resolve(__dirname, "./src/pages"),
      "@src": resolve(__dirname, "./src"),
      "@util": resolve(__dirname, "./src/util"),
      "msw/browser": resolve(
        __dirname,
        "node_modules/msw/lib/browser/index.mjs"
      ),
    },
  },
  server: {
    port: 4173,
  },
});
