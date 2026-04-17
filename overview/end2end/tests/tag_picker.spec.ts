// Leptodon
//
// Copyright (C) 2025-2026 Open Analytics NV
//
// ===========================================================================
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the Apache License as published by The Apache Software
// Foundation, either version 2 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the Apache License for more details.
//
// You should have received a copy of the Apache License along with this program.
// If not, see <http://www.apache.org/licenses/>
import { test, expect } from "@playwright/test";

// Helper function to get input locator for a tag by text
function getTagInputLocator(tag_content: any, tagText: string) {
  return tag_content.locator("div").filter({ hasText: tagText }).locator("input").locator("visible=true");
}

// Check that the radio reactively updates its signal.
test("Tag Picker functionality", async ({ page }) => {
  await page.goto("/test_tag_picker");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Tag Picker");

  let tag_picker = page.locator("#tag_picker");
  let sel_disp = page.locator("#selected-display");
  let tag_trigger = page.locator("#tag_picker-trigger");
  let tag_content = page.locator("#tag_picker-content");
  let tag_search = page.locator("#tag_picker-search");
  let btn_set_5 = page.locator("#set-5");
  let btn_set_none = page.locator("#set-none");
  let btn_elems_3_10 = page.locator("#elems-3-10");

  await expect(sel_disp).toHaveText("");
  await tag_picker.click();
  await expect(tag_search).toBeFocused();

  // Test keyboard selection
  await page.keyboard.press("H");
  await page.keyboard.press("y");
  await page.keyboard.press("Enter");
  await expect(sel_disp).toHaveText("Hydrogen");
  // Should be checked now in the dropdown.
  await expect(getTagInputLocator(tag_content, "Hydrogen")).toBeChecked();
  await page.keyboard.press("Backspace");
  await page.keyboard.press("e");
  await page.keyboard.press("Enter");
  // Should be checked now in the dropdown.
  await expect(getTagInputLocator(tag_content, "Helium")).toBeChecked();
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
  await tag_content.locator("div").filter({ hasText: "Boron" }).click();
  await expect(getTagInputLocator(tag_content, "Boron")).toBeChecked();
  await expect(sel_disp).toHaveText("Boron");

  await getTagInputLocator(tag_content, "Carbon").click();
  await expect(getTagInputLocator(tag_content, "Carbon")).toBeChecked();
  await expect(sel_disp).toHaveText("BoronCarbon");

  // Deselecting
  await getTagInputLocator(tag_content, "Carbon").click();
  await expect(getTagInputLocator(tag_content, "Carbon")).not.toBeChecked();
  await expect(sel_disp).toHaveText("Boron");

  // Test selection changes from outside the tagpicker.
  await btn_set_none.click();
  await expect(sel_disp).toHaveText("");

  await btn_set_5.click();
  await expect(sel_disp).toHaveText("BerylliumBoronCarbon");

  // Open tag-picker dropdown
  tag_trigger.click();
  await expect(tag_content).toBeInViewport();
  await expect(tag_content).toBeVisible();
  // Remove lithium using the tag-picker trigger.
  await tag_trigger
    .locator("div")
    .locator("div")
    .filter({ hasText: "Beryllium" })
    .locator("div")
    .click();

  await expect(tag_content).toBeVisible();

  // Validate it has been removed from the output
  await expect(sel_disp).toHaveText("BoronCarbon");
  // Should be removed from the tag-picker trigger.
  expect(
    tag_trigger.locator("div").locator("div").filter({ hasText: "Beryllium" }),
  ).toHaveCount(0);
  // Should be unchecked now in the dropdown.
  await expect(getTagInputLocator(tag_content, "Beryllium")).not.toBeChecked();
});

// Check that the radio reactively updates its signal.
test("Tag Picker keyboard-navigation", async ({ page }) => {
  await page.goto("/test_tag_picker");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Tag Picker");

  let tag_picker = page.locator("#tag_picker");
  let sel_disp = page.locator("#selected-display");
  let tag_trigger = page.locator("#tag_picker-trigger");
  let tag_content = page.locator("#tag_picker-content");
  let tag_search = page.locator("#tag_picker-search");

  page.keyboard.press("Tab"); // selects tag_picker

  await expect(tag_trigger).toBeFocused();

  page.keyboard.press("Enter"); // open it

  await expect(tag_content).toBeVisible();
  await expect(tag_search).toBeFocused(); // should the search-input

  page.keyboard.press("Escape"); // close it

  // Focus should transfer back to tag_picker when closing via keyboard.
  await expect(tag_trigger).toBeFocused();
});

// LLM QWEN3:30b generated test, only took a minimal look at it.
test("Tag Picker dropdown opens without scrolling the page", async ({
  page,
}) => {
  await page.goto("/test_tag_picker");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Tag Picker");

  // Bring tag picker into view
  await page.locator("#tag_picker").hover();

  // Record initial scroll position
  const { scrollX: initialX, scrollY: initialY } = await page.evaluate(() => ({
    scrollX: window.scrollX,
    scrollY: window.scrollY,
  }));

  // Click to open dropdown
  await page.locator("#tag_picker").click();

  // Wait for dropdown to be visible
  await page.waitForSelector("#tag_picker-content", { state: "visible" });

  // Verify page didn't move during dropdown opening
  const { scrollX: finalX, scrollY: finalY } = await page.evaluate(() => ({
    scrollX: window.scrollX,
    scrollY: window.scrollY,
  }));

  expect(finalX).toBe(initialX);
  expect(finalY).toBe(initialY);
});
