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

// Tabs should swap their tab content.
test("Tabs functionality", async ({ page }) => {
  await page.goto("/test_tabs");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Tabs");

  const profile_tab = page.getByText("Profile", { exact: true });
  const settings_tab = page.getByText("Settings", { exact: true });

  await profile_tab.click();
  await expect(page.locator("#profile-content")).toBeVisible();
  await expect(page.locator("#settings-content")).toHaveCount(0);

  await settings_tab.click();
  await expect(page.locator("#profile-content")).toHaveCount(0);
  await expect(page.locator("#settings-content")).toBeVisible();
});
