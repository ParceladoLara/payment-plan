import { defineConfig } from "@playwright/test";

export default defineConfig({
  webServer: {
    command: "npm run build:vite && npm run serve:vite",
    port: 4173,
    reuseExistingServer: !process.env.CI,
  },
  testDir: "./playwright",
});
