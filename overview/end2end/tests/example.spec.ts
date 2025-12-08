import { test, expect } from "@playwright/test";

// Sanity check
test("homepage has title 'Leptos components'", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await expect(page).toHaveTitle("Leptos components");
});