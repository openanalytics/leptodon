import { test, expect } from "@playwright/test";

// Buttons are interactive
test("Button on_click", async ({ page }) => {
  await page.goto("http://localhost:3000/");
  
  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Leptos components");
  
  await page.getByRole('button', { name: '2', exact: true }).click();
  await page.getByRole('button', { name: '4', exact: true }).click();
  await page.getByRole('button', { name: '8', exact: true }).click();

});