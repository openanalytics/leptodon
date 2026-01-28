import { test, expect } from "@playwright/test";

// Check that the select reactively updates its signal.
test("Radio button functionality", async ({ page }) => {
  await page.goto("http://localhost:3000/test_select");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Select");

  let btn_set_5 = page.locator("#set-5");
  let btn_set_none = page.locator("#set-none");
  let btn_elems_1_7 = page.locator("#elems-1-7");
  let btn_elems_3_10 = page.locator("#elems-3-10");

  
});
