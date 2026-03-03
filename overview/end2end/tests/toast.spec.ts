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

/// Tests basic display and functionality of the toast.
test("test toast", async ({ page }) => {
  await page.goto("http://localhost:3000/test_toast");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Toast");

  let permanent_toast = page.locator("#permanent-toast");
  let dismissable_toast = page.locator("#dismissable-toast");
  let detailed_toast = page.locator("#detailed-toast");

  await expect(permanent_toast).toHaveCount(0);
  await expect(dismissable_toast).toHaveCount(0);
  await expect(detailed_toast).toHaveCount(0);

  await page.getByRole("button").click();

  await expect(permanent_toast).toHaveCount(1);
  await expect(dismissable_toast).toHaveCount(1);
  await expect(detailed_toast).toHaveCount(1);

  await expect(permanent_toast.getByText("Permanent toast")).toBeVisible();
  await expect(permanent_toast.getByRole("button")).toHaveCount(0);

  await expect(dismissable_toast.getByText("Dismissable toast")).toBeVisible();
  await expect(dismissable_toast.getByText("Don't forget to drink water!")).toBeVisible();
  await dismissable_toast.getByRole("button").click();
  await expect(dismissable_toast).toHaveCount(0);

  await expect(detailed_toast.getByText("Detailed toast")).toBeVisible();
  await expect(detailed_toast.getByText("Details")).toBeVisible();
});
