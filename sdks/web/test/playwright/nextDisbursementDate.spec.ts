import { test, expect } from "@playwright/test";

test.describe("NextDisbursementDate page", () => {
  test("should render message", async ({ page }) => {
    const expectedText = `"2078-02-16T03:00:00.000Z"`;
    await page.goto("/");
    await page.click('button:has-text("Next Disbursement Date")');
    await expect(page.locator("pre")).toContainText(expectedText.trim());
  });
});
