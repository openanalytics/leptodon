import { test, expect } from "@playwright/test";

// Sanity check
test("Dropdown opens and closes.", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Leptos components");

  // Click the get started link.
  await page.getByText("DropDownButton").click();

  // Expects page to have a heading with the name of Installation.
  await expect(page.getByText("Entry-1")).toBeVisible();
});
