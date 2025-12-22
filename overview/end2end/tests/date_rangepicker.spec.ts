import { test, expect } from "@playwright/test";

// Test date-picker open/close on click
test("Test date-picker open/close", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Leptos components");

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
  await expect(page).toHaveTitle("Leptos components");

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
test("Test date-picker selecting open/close", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Leptos components");

  await expect(page.locator("#date_range_picker-left-popup")).toBeHidden();
  await expect(page.locator("#date_range_picker-right-popup")).toBeHidden();

  await page.locator('#date_range_picker-left').click();
  await expect(page.locator("#date_range_picker-left-popup")).toBeVisible();
  await expect(page.locator("#date_range_picker-right-popup")).toBeHidden();
  
  await page.locator('#date_range_picker-left-popup').getByText('18').click();
  await expect(page.locator("#date_range_picker-left-popup")).toBeHidden();
  await expect(page.locator("#date_range_picker-right-popup")).toBeHidden();

  await page.locator('#date_range_picker-right').click();
  await expect(page.locator("#date_range_picker-right-popup")).toBeVisible();
  await expect(page.locator("#date_range_picker-left-popup")).toBeHidden();
  
  await page.locator('#date_range_picker-right-popup').getByText('19').click();
  await expect(page.locator("#date_range_picker-left-popup")).toBeHidden();
  await expect(page.locator("#date_range_picker-right-popup")).toBeHidden();
});

test("Test date-picker functionality", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Leptos components");

  // Select 2135-10-01 in left date-input
  await page.locator("#date_range_picker-left").click();
  await page.getByRole("button", { name: "December" }).click();
  await page.getByRole("button", { name: "2025" }).click();
  await page.getByRole("button", { name: "- 2029" }).click();
  await page.getByRole("button", { name: "- 2090" }).click();
  await page
    .locator(
      "#date_range_picker-left-popup > .inline-block > .datepicker-header > .flex > button:nth-child(3)",
    )
    .first()
    .click();
  await page.getByText("2130").click();
  await page.getByText("2135").click();
  await page.getByText("Oct").click();
  await page.getByText("1", { exact: true }).nth(3).click();

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
