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
