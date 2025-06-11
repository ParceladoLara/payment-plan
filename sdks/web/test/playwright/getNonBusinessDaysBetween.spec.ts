import { test, expect } from "@playwright/test";

test.describe("GetNonBusinessDaysBetween page", () => {
  test("should render message", async ({ page }) => {
    const expectedText = `[
  "2078-11-12T03:00:00.000Z",
  "2078-11-13T03:00:00.000Z",
  "2078-11-15T03:00:00.000Z",
  "2078-11-19T03:00:00.000Z",
  "2078-11-20T03:00:00.000Z"
]`;
    await page.goto("http://localhost:3000");
    await page.click('button:has-text("Get Non Business Days Between")');
    await expect(page.locator("pre")).toContainText(expectedText.trim());
  });
});
