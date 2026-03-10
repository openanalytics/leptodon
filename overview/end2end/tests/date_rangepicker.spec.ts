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
import { test, expect, Page } from "@playwright/test";

// Test date-picker open/close on click
test("Test date-picker open/close", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Leptodon");

  await expect(page.locator("#date_range_picker-left-popup")).toBeHidden();
  await page.locator("#date_range_picker-left").click();
  await expect(page.locator("#date_range_picker-left-popup")).toBeVisible();
  await page.locator("body").click();
  await expect(page.locator("#date_range_picker-left-popup")).toBeHidden();
});

// Test date-picker open/close on focus/tab
test("Test date-picker focus/tab open/close", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Leptodon");

  await expect(page.locator("#date_range_picker-left-popup")).toBeHidden();
  await expect(page.locator("#date_range_picker-right-popup")).toBeHidden();

  await page.locator("#date_range_picker-left").focus();
  await expect(page.locator("#date_range_picker-left-popup")).toBeVisible();
  await expect(page.locator("#date_range_picker-right-popup")).toBeHidden();

  await page.locator("#date_range_picker-left").press("Tab");
  await expect(page.locator("#date_range_picker-right")).toBeFocused();
  await expect(page.locator("#date_range_picker-right-popup")).toBeVisible();
  await expect(page.locator("#date_range_picker-left-popup")).toBeHidden();
});

// Test date-picker open/close on selecting
async function testDatePickerOpenClosing(page: Page) {
  await expect(page.locator("#date_range_picker-left-popup")).toBeHidden();
  await expect(page.locator("#date_range_picker-right-popup")).toBeHidden();

  await page.locator("#date_range_picker-left").click();
  await expect(page.locator("#date_range_picker-left-popup")).toBeVisible();
  await expect(page.locator("#date_range_picker-right-popup")).toBeHidden();

  await page.locator("#date_range_picker-left-popup").getByText("18").click();
  await expect(page.locator("#date_range_picker-left-popup")).toBeHidden();
  await expect(page.locator("#date_range_picker-right-popup")).toBeHidden();

  await page.locator("#date_range_picker-right").click();
  await expect(page.locator("#date_range_picker-right-popup")).toBeVisible();
  await expect(page.locator("#date_range_picker-left-popup")).toBeHidden();

  await page.locator("#date_range_picker-right-popup").getByText("19").click();
  await expect(page.locator("#date_range_picker-left-popup")).toBeHidden();
  await expect(page.locator("#date_range_picker-right-popup")).toBeHidden();
}

test("Test date-picker selecting open/close", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Leptodon");

  await testDatePickerOpenClosing(page);
});

// Added because labels can interfere with focus and we want to catch regression in this area.
test("Test labeled date-picker selecting open/close", async ({ page }) => {
  await page.goto("http://localhost:3000/forms");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Forms");

  await testDatePickerOpenClosing(page);
});

// Just date-picking functionality
test("Test date-picker functionality", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Leptodon");

  // Select 2135-10-01 in left date-input
  let current_date = new Date();
  let month = current_date.toLocaleString("en-US", { month: "long" });
  let year = current_date.toLocaleString("en-US", { year: "numeric" });
  let decenium = current_date
    .toLocaleString("en-US", { year: "numeric" })
    .replace(/.$/, "9");
  let millenium = current_date
    .toLocaleString("en-US", { year: "numeric" })
    .replace(/..$/, "90");
  await page.locator("#date_range_picker-left").click();
  await page.getByRole("button", { name: month }).click();
  await page.getByRole("button", { name: year }).click();
  await page.getByRole("button", { name: `- ${decenium}` }).click();
  await page.getByRole("button", { name: `- ${millenium}` }).click();
  await page
    .locator(
      "#date_range_picker-left-popup > .inline-block > .datepicker-header > .flex > button:nth-child(3)",
    )
    .first()
    .click();
  await page.getByText("2130").click();
  await page.getByText("2135").click();
  await page.getByText("Oct").click();
  await page
    .locator(
      "#date_range_picker-left-popup",
    )
    .first()
    .getByText("1", { exact: true })
    .first()
    .click();

  // Select The 20th of current month.
  await page.locator("#date_range_picker-right").click();
  await page
    .locator(
      "#date_range_picker-right-popup > .inline-block > .datepicker-header > .flex > button:nth-child(1)",
    )
    .first()
    .click();
  await page
    .locator("#date_range_picker-right-popup")
    .getByText("20", { exact: true })
    .first()
    .click();

  // Test dates being swapped after selecting the later date on the left.
  await expect(page.locator("#date_range_picker-right")).toHaveValue(
    "2135-10-01",
  );
});

// Reactivity
test("Test date-picker reactivity", async ({ page }) => {
  await page.goto('http://127.0.0.1:3000/test_date_range_picker');

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test DateRangePicker");

  const start_date_input = page.getByRole('textbox', { name: 'yyyy-mm-dd' }).first();
  const end_date_input = page.getByRole('textbox', { name: 'yyyy-mm-dd' }).nth(1);

  // Assert the intial dates are visible in the date-inputs.
  await page.getByText('Some(2025-10-10) - Some(2025-11-15)').click();

  await expect(start_date_input).toHaveValue("2025-10-10");
  await expect(end_date_input).toHaveValue("2025-11-15");
  await page.locator('html').click();

  await page.getByRole('button', { name: 'Clear' }).click();
  await expect(start_date_input).toHaveValue("");
  await expect(end_date_input).toHaveValue("");
  await page.getByText('None - None').click();

  await page.getByRole('button', { name: 'Set 2010 as start_date' }).click();
  await page.getByText('None - None').click();
  await page.getByRole('textbox', { name: 'yyyy-mm-dd' }).first().click();
  await page.getByRole('button', { name: 'Set 2025 as start_date' }).click();
  await page.getByText('Some(2025-10-10) - None').click();
  await page.getByRole('button', { name: 'Set 2026 as end_date' }).click();
  await page.getByRole('textbox', { name: 'yyyy-mm-dd' }).nth(1).click();
  await page.getByText('Some(2025-10-10) - Some(2026-').click();
});