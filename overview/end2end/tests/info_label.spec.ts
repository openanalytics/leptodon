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

test("Info Label functionality", async ({ page }) => {
  await page.goto("/test_info_label");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Info Label");

  const text1 = page.getByTestId("text-1-with-info-label");
  const text2 = page.getByTestId("text-2-with-info-label");
  const button1 = text1.locator("button");
  const button2 = text2.locator("button");
  const label1 = button1.locator(".info-label");
  const label2 = button2.locator(".info-label");


  await expect(label1).toBeHidden();
  await expect(label2).toBeHidden();

  await button1.hover();
  await expect(label1).toBeVisible();
  await expect(label2).toBeHidden();

  await button2.hover();
  await expect(label2).toBeVisible();
  await expect(label1).toBeHidden();

  await label2.hover();
  await expect(label2).toBeVisible();
  await expect(label1).toBeHidden();
});
