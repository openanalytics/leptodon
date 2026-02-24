import { test, expect } from "@playwright/test";

// Buttons are interactive
test("Button on_click", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Leptodon");

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

// CopyButtons are reactive and should function
test("CopyButton functionality", async ({ page, context }) => {
  await page.goto("http://localhost:3000/test_copy_button");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test CopyButton");

  const copy_btn = page.locator("#copy-button");
  const textarea = page.getByRole('textbox', { name: 'Test paste here' });

  await page.locator("#set-test-string1").click();
  await copy_btn.click();
  await textarea.click();
  await page.keyboard.press("Control+V");
  await expect(textarea).toHaveValue("test_string1");

  await page.locator("#set-test-string2").click();
  await copy_btn.click();
  await textarea.click();
  await page.keyboard.press("Control+V");
  await expect(textarea).toHaveValue("test_string1test_string2");
});
