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

/// Tests basic display and functionality of the calendar.
test("test calendar", async ({ page }) => {
  await page.goto("http://localhost:3000/test_calendar");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Calendar");

  // Mock date
  await page.clock.install({ time: new Date("2026-02-02T00:00:00Z")  });

  // Assert current mocked date
  await page.getByText("February").click();
  await page.locator("div").filter({ hasText: "2" }).nth(3).click();
  await page.locator("div").filter({ hasText: "27" }).nth(2).click();
  await page.locator("div").filter({ hasText: "6" }).nth(3).click();
  await page.getByText("Mon", { exact: true }).click();
  await page.getByText("Tue", { exact: true }).click();
  await page.getByText("Wed", { exact: true }).click();
  await page.getByText("Thu", { exact: true }).click();
  await page.getByText("Fri", { exact: true }).click();

  await page.getByText("Current Month").click();
  // Navigate to prev month
  await page.getByRole("button").first().click();

  await page.getByText("January").click();
  await page.getByText("Mon", { exact: true }).click();
  await page.getByText("Tue", { exact: true }).click();
  await page.getByText("Wed", { exact: true }).click();
  await page.getByText("Thu", { exact: true }).click();
  await page.getByText("Fri", { exact: true }).click();

  // Check for 3 empty slots because they're from previous month.
  await page.locator(".bg-gray-100").first().click();
  await page.locator(".bg-gray-100").nth(1).click();
  await page.locator(".bg-gray-100").nth(2).click();

  // Check edge cases.
  await page.locator("div").filter({ hasText: /^1$/ }).click();
  await page.locator("div").filter({ hasText: /^2$/ }).click();
  await page.locator("div").filter({ hasText: /^5$/ }).first().click();
  await page.locator("div").filter({ hasText: "30" }).nth(2).click();
  await page.locator("div").filter({ hasText: "9" }).nth(2).click();
  await page.locator("div").filter({ hasText: "26" }).nth(3).click();

  // Go back to current month.
  await page.getByRole("button", { name: "back to February" }).click();
  await page.getByText("February").click();
});
