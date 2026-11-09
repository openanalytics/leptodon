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

// Checkboxs are interactive
test("Checkbox interactivity", async ({ page }) => {
  await page.goto("/test_checkbox");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Checkbox");

  // Test generic button functionality
  // div around checkbox
  let checkbox = page.getByTestId("checkbox");
  let checkbox_disp = page.getByTestId("checkbox-disp");
  // <input> elem
  let checkbox_input = checkbox.getByRole("checkbox");
  let btn_on = page.getByTestId("btn-on");
  let btn_off = page.getByTestId("btn-off");

  async function check_checkbox_state(state: boolean) {
    if (state) {
      await expect(checkbox_disp).toHaveText("true");
      await expect(checkbox_input).toBeChecked();
    } else {
      await expect(checkbox_disp).toHaveText("false");
      await expect(checkbox_input).not.toBeChecked();
    }
  }

  await check_checkbox_state(true);
  await btn_off.click();
  await check_checkbox_state(false);
  await btn_on.click();
  await check_checkbox_state(true);
  await checkbox_input.click();
  await check_checkbox_state(false);
  await checkbox.locator("label").click();
  await check_checkbox_state(true);
});
