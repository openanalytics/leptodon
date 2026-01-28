import { test, expect } from "@playwright/test";

// Check that the select reactively updates its signal.
test("Select functionality", async ({ page, browserName }) => {
  // test.skip(
  //   browserName === "firefox",
  //   "https://projects.openanalytics.eu/issues/36185",
  // );
  await page.goto("http://localhost:3000/test_select");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Select");

  let sel_disp = page.locator("#selected-display");
  let sel = page.locator("#sel");
  let btn_set_5 = page.locator("#set-5");
  let btn_set_none = page.locator("#set-none");
  let btn_elems_1_7 = page.locator("#elems-1-7");
  let btn_elems_3_10 = page.locator("#elems-3-10");

  // Initial state
  await expect(sel_disp).toHaveText("");
  await expect(sel).toHaveValue("");

  await sel.selectOption("2");
  await expect(sel_disp).toHaveText("2");
  await expect(sel).toHaveValue("2");

  await btn_set_5.click();
  await expect(sel_disp).toHaveText("5");
  await expect(sel).toHaveValue("5");

  await btn_set_none.click();
  await expect(sel_disp).toHaveText("");
  await expect(sel).toHaveValue("");

  // Change possible elements to any integer in (1..=7)
  await btn_elems_1_7.click();

  await sel.selectOption("1");
  await expect(sel_disp).toHaveText("1");
  await expect(sel).toHaveValue("1");

  // Selected option no longer exists, should deselect.
  await btn_elems_3_10.click();
  await expect(sel_disp).toHaveText("");
  await expect(sel).toHaveValue("");

  await sel.selectOption("3");
  await btn_elems_1_7.click();
  // 3 Does exist in the new elems set, should stay selected.
  await expect(sel_disp).toHaveText("3");
  await expect(sel).toHaveValue("3");
});
