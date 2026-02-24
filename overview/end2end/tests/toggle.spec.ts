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

// Toggles are interactive
test("Toggle interactivity", async ({ page }) => {
  await page.goto("http://localhost:3000/test_toggle");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Toggle");

  // Test generic button functionality
  let toggle_disp = page.getByTestId("toggle-disp");
  let toggle = page.getByTestId("toggle");
  let btn_on = page.getByTestId("btn-on");
  let btn_off = page.getByTestId("btn-off");

  async function check_toggle_state(state: boolean) {
    if (state) {
      await expect(toggle_disp).toHaveText("true");
      await expect(toggle).toBeChecked();
    } else {
      await expect(toggle_disp).toHaveText("false");
      await expect(toggle).not.toBeChecked();
    }
  }

  await check_toggle_state(true);
  await btn_off.click();
  await check_toggle_state(false);
  await btn_on.click();
  await check_toggle_state(true);
  await toggle.click();
  await check_toggle_state(false);
  await toggle.locator("span").click();
  await check_toggle_state(true);
});
