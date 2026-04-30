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

/// Tests basic display functionality and reactivity of the alert.
test("test alerts", async ({ page }) => {
  await page.goto("/test_alert");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Alerts");

  let alert = page.getByTestId("test-alert");
  let alert_dismiss_btn = alert.getByRole("button").last();
  let btn_change_content = page.getByTestId("btn-change-content");
  let dismissed_status = page.getByTestId("dismissed-status");

  await expect(alert).toBeVisible();
  await expect(alert_dismiss_btn).toBeVisible();
  await expect(dismissed_status).toHaveText("false");

  await expect(alert).toHaveText("prefix - test-string");
  await btn_change_content.click();
  await expect(alert).toHaveText("prefix - 🐈");

  await alert_dismiss_btn.click();
  await expect(alert).toBeHidden();

  await expect(dismissed_status).toHaveText("true");
});
