import { test, expect, Page, Locator } from "@playwright/test";

// Check that the radio reactively updates its signal.
test("Tag Picker functionality", async ({ page, browserName }) => {
  await page.goto("http://localhost:3000/test_tag_picker");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Tag Picker");

  let tag_picker = page.locator("#tag_picker");
  let sel_disp = page.locator("#selected-display");
  let tag_trigger = page.locator("#tag_picker-trigger");
  let tag_dropdown = page.locator("#tag_picker-dropdown");
  let btn_set_5 = page.locator("#set-5");
  let btn_set_none = page.locator("#set-none");
  let btn_elems_3_10 = page.locator("#elems-3-10");

  await expect(sel_disp).toHaveText("");
  await tag_picker.click();

  // Test keyboard selection
  await page.keyboard.press("H");
  await page.keyboard.press("y");
  await page.keyboard.press("Enter");
  await expect(sel_disp).toHaveText("Hydrogen");
  await page.keyboard.press("Backspace");
  await page.keyboard.press("e");
  await page.keyboard.press("Enter");
  await expect(sel_disp).toHaveText("HydrogenHelium");
  await page.keyboard.press("Backspace");
  await page.keyboard.press("Backspace");
  await page.keyboard.press("Escape");

  // Selected elems 1 and 2
  // Check that the tags are present in the tagpicker
  expect(
    tag_trigger.locator("div").filter({ hasText: "Hydrogen" }).first(),
  ).toBeDefined();

  // Exclude hydrogen from possible tags
  await btn_elems_3_10.click();
  // Check that it gets deselected.
  await expect(sel_disp).toHaveText("");

  // Re-open
  await tag_picker.click();
  // Test mouse selection
  await tag_dropdown.locator("li").filter({ hasText: "Boron" }).click();
  await expect(
    tag_dropdown.locator("li").filter({ hasText: "Boron" }).locator("input"),
  ).toBeChecked();
  await expect(sel_disp).toHaveText("Boron");

  await tag_dropdown
    .locator("li")
    .filter({ hasText: "Carbon" })
    .locator("input")
    .click();
  await expect(
    tag_dropdown.locator("li").filter({ hasText: "Carbon" }).locator("input"),
  ).toBeChecked();
  await expect(sel_disp).toHaveText("BoronCarbon");

  // Deselecting
  await tag_dropdown
    .locator("li")
    .filter({ hasText: "Carbon" })
    .locator("input")
    .click();
  await expect(
    tag_dropdown.locator("li").filter({ hasText: "Carbon" }).locator("input"),
  ).not.toBeChecked();
  await expect(sel_disp).toHaveText("Boron");

  // Test selection changes from outside the tagpicker.
  await btn_set_none.click();
  await expect(sel_disp).toHaveText("");

  await btn_set_5.click();
  await expect(sel_disp).toHaveText("BerylliumBoronCarbon");

  // Open tag-picker dropdown
  tag_trigger.click();
  await expect(tag_dropdown).toBeInViewport();
  // Remove lithium using the tag-picker trigger.
  await tag_trigger
    .locator("div")
    .locator("div")
    .filter({ hasText: "Beryllium" })
    .locator("div")
    .click();

  await expect(tag_dropdown).toBeVisible();

  // Validate it has been removed from the output
  await expect(sel_disp).toHaveText("BoronCarbon");
  // Should be removed from the tag-picker trigger.
  expect(
    tag_trigger.locator("div").locator("div").filter({ hasText: "Beryllium" }),
  ).toHaveCount(0);
  // Should be unchecked now in the dropdown.
  await expect(
    tag_dropdown.locator("li").filter({ hasText: "Beryllium" }).locator("input"),
  ).not.toBeChecked();
});
