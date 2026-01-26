import { test, expect } from "@playwright/test";

// Buttons are interactive
test("Button on_click", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Leptos components");

  // Test generic button functionality
  await page.getByRole("button", { name: "2", exact: true }).click();
  await page.getByRole("button", { name: "4", exact: true }).click();
  await page.getByRole("button", { name: "8", exact: true }).click();

  // Test controlled button functionality 
  let controlled_number = page.locator("#controlled_number_input");
  let input_elem = controlled_number.locator("input").first();
  let decrement_btn = controlled_number.getByTestId("decrement");
  let increment_btn = controlled_number.getByTestId("increment");

  // Starts empty ~ 0
  await expect(input_elem).toHaveValue("");

  // +2
  await increment_btn.click();
  await increment_btn.click();
  
  await expect(input_elem).toHaveValue("2");

  // -3
  await decrement_btn.click();
  await decrement_btn.click();
  await decrement_btn.click();

  await expect(input_elem).toHaveValue("-1");
});
