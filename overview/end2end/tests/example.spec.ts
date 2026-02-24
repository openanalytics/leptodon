import { test, expect } from "@playwright/test";

// Sanity check
test("homepage has title 'Leptodon'", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await expect(page).toHaveTitle("Leptodon");
});
