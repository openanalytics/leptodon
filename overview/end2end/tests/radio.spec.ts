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
import { test, expect, Page, Locator } from "@playwright/test";

// Check that the radio buttons can be clicked on their label, surrounding div etc.
test("Radio button functionality", async ({ page }) => {
  await page.goto("http://localhost:3000/forms");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Forms");

  let radio_input = page.getByTestId("radio-input");
  // Check for radio label
  await radio_input.getByText("*Radio Stations").click();

  let options = radio_input.locator("ul");

  // Click surrounding li.
  await options.locator("li").filter({ hasText: "Radio1" }).click();
  await expect(options.getByLabel("Radio1", { exact: true })).toBeChecked();
  // Click label.
  await options.getByText("Radio2").click();
  await expect(options.getByLabel("Radio2", { exact: true })).toBeChecked();
  // Click radio orb.
  await options.getByRole("radio", { name: "Klara", exact: true }).check();
  await expect(options.getByLabel("Klara", { exact: true })).toBeChecked();
});

// Check that the radio reactively updates its signal.
test("Radio functionality", async ({ page, browserName }) => {
  await page.goto("http://localhost:3000/test_radio");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Radio");

  function nth_radio(page: Page, n: number): Locator {
    return page.locator("#radio-" + n.toString());
  }

  async function expect_all_unselected(
    page: Page,
    start: number = 1,
    end: number = 10,
  ) {
    for (let i = start; i <= end; i++) {
      await expect(nth_radio(page, i)).not.toBeChecked();
    }
  }

  let sel_disp = page.locator("#selected-display");
  let btn_set_5 = page.locator("#set-5");
  let btn_set_none = page.locator("#set-none");
  let btn_elems_1_7 = page.locator("#elems-1-7");
  let btn_elems_3_10 = page.locator("#elems-3-10");

  // Initial state
  await expect(sel_disp).toHaveText("");
  await expect_all_unselected(page);

  await nth_radio(page, 2).setChecked(true);
  await expect(sel_disp).toHaveText("2");
  await expect(nth_radio(page, 2)).toBeChecked();

  await btn_set_5.click();
  await expect(sel_disp).toHaveText("5");
  await expect(nth_radio(page, 5)).toBeChecked();

  await btn_set_none.click();
  await expect(sel_disp).toHaveText("");
  await expect_all_unselected(page);

  // Change possible elements to any integer in (1..=7)
  await btn_elems_1_7.click();
  await expect_all_unselected(page, 1, 7);

  await nth_radio(page, 1).click();
  await expect(sel_disp).toHaveText("1");
  await expect(nth_radio(page, 1)).toBeChecked();

  // Selected option no longer exists, should deselect.
  await btn_elems_3_10.click();
  await expect(sel_disp).toHaveText("");
  await expect_all_unselected(page, 3, 10);

  await nth_radio(page, 3).click();
  await btn_elems_1_7.click();
  // 3 Does exist in the new elems set, should stay selected.
  await expect(sel_disp).toHaveText("3");
  await expect(nth_radio(page, 3)).toBeChecked();
});
