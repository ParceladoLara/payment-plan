import { test, expect } from "@playwright/test";

test.describe("DisbursementDateRange page", () => {
  test("should render message", async ({ page }) => {
    const expectedText = `[
  "2078-02-16T03:00:00.000Z",
  "2078-02-22T03:00:00.000Z"
]`;
    await page.goto("/");
    await page.click('button:has-text("Disbursement Date Range")');
    await expect(page.locator("pre")).toContainText(expectedText.trim());
  });
});
