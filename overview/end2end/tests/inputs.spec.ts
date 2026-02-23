import { test, expect, Page, Locator } from "@playwright/test";

// Check that the value signal is properly updated and displayed when inputting.
test("Input signal behaviours", async ({ page }) => {
  await page.goto("http://localhost:3000/test_inputs");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Inputs");

  let text_disp = page.locator("#text-input-display");
  // No minimum, max 10 chars
  let text_input = page.locator("#text-input");


  await text_input.click();
  await expect(text_input).toBeEmpty();
  await expect(text_disp).toBeEmpty();

  page.keyboard.press("H");
  await expect(text_disp).toHaveText("H");

  for (let i = 0; i < 15; i++) {
    page.keyboard.press("i");
  }

  // Max 10 chars
  await expect(text_disp).toHaveText("Hiiiiiiiii");
  await expect(text_input).toHaveValue("Hiiiiiiiiiiiiiii");

  await text_input.clear();
  await expect(text_disp).toHaveText("");
  await expect(text_input).toHaveValue("");
});

// Check that only integers in the (0,10) range are accepted
test("Integer number input", async ({ page }) => {
  await page.goto("http://localhost:3000/test_inputs");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Inputs");

  let number_disp = page.locator("#u32-input-display");
  let number_input = page.locator("#u32-input");

  await number_input.press("0"); // 0
  await expect(number_disp).toHaveText("0");

  await number_input.press("ArrowUp"); // 1
  await expect(number_disp).toHaveText("1");
  await number_input.press("1"); // 11
  await expect(number_disp).toHaveText("1");

  let number_input_invalid = page.locator("#u32-input-invalid-reason");
  await expect(number_input_invalid).toContainText("10"); // Input needs to be <= 10

  await number_input.clear(); // ""
  await expect(number_input_invalid).toContainText("integer"); // Required input should tell you to input something.

  await number_input.press("-"); // -
  await number_input.press("1"); // -1
  await expect(number_input_invalid).toContainText("positive");
});

// Check that only integers in the (-100,10) range are accepted
test("Negative integer number input", async ({ page }) => {
  await page.goto("http://localhost:3000/test_inputs");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Inputs");

  let number_disp = page.locator("#i128-input-display");
  let number_input = page.locator("#i128-input");

  await number_input.press("0"); // 0
  await expect(number_disp).toHaveText("0");

  await number_input.press("ArrowUp"); // 1
  await expect(number_disp).toHaveText("1");
  await number_input.press("1"); // 11
  await expect(number_disp).toHaveText("1");

  let number_input_invalid = page.locator("#i128-input-invalid-reason");
  await expect(number_input_invalid).toContainText("10"); // Input needs to be <= 10

  await number_input.clear(); // ""
  await expect(number_input_invalid).toHaveCount(0); // Optional input should not complain when empty.

  await number_input.press("-"); // -
  await number_input.press("1"); // -1
  await expect(number_input_invalid).toHaveCount(0); // -1 is valid
  await number_input.press("1"); // -11
  await number_input.press("1"); // -111

  await expect(number_input_invalid).toContainText("-100"); // -111 is out of range.
});


// Check that only decimals in the (-2.00,10.15) range are accepted
test("Decimal number input", async ({ page }) => {
  await page.goto("http://localhost:3000/test_inputs");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Inputs");

  let number_disp = page.locator("#f64-input-display");
  let number_input = page.locator("#f64-input");

  await number_input.press("0"); // 0
  await expect(number_disp).toHaveText("0");

  await number_input.press("ArrowUp"); // 1
  await expect(number_disp).toHaveText("0.01");
  let number_input_invalid = page.locator("#f64-input-invalid-reason");

  number_input.clear();
  await expect(number_input_invalid).toHaveCount(0);   // Optional input should not complain when empty.
  await number_input.press("-"); // -3
  await number_input.press("3"); // -3
  await expect(number_input_invalid).toContainText("-2"); // Input needs to be >= -2
});
