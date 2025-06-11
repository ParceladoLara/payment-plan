import { defineConfig } from "@playwright/test";

export default defineConfig({
  webServer: {
    command: "npm run build:rsbuild && npm run serve:rsbuild",
    port: 4173,
    reuseExistingServer: !process.env.CI,
  },
  testDir: "./playwright",
});
