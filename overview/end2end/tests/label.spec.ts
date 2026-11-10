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

// Clicks the label with the given name and checks that the input gains focus.
async function click_label_expect_focus(
  page: Page,
  label_name: RegExp,
  input: Locator,
) {
  await expect(input).not.toBeFocused();
  await page.getByText(label_name).click();
  await expect(input).toBeFocused();
}

// Clicks the label with the given name and checks that the checkbox-like input toggles.
async function click_label_expect_toggle(
  page: Page,
  label_name: RegExp,
  input: Locator,
) {
  let initial = await input.isChecked();
  await page.getByText(label_name).click();
  if (initial) {
    await expect(input).not.toBeChecked();
  } else {
    await expect(input).toBeChecked();
  }
}

// Check that clicking a standalone text-input's label focuses the input.
test("Label click focuses labeled text input", async ({ page }) => {
  await page.goto("/test_inputs");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Inputs");

  await click_label_expect_focus(
    page,
    /Labeled text input/,
    page.locator("#labeled-text-input"),
  );
});

// Check that clicking a standalone password-input's label focuses the input.
test("Label click focuses labeled password input", async ({ page }) => {
  await page.goto("/test_inputs");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Inputs");

  await click_label_expect_focus(
    page,
    /Labeled password input/,
    page.locator("#labeled-password-input"),
  );
});

// Check that clicking a standalone textarea's label focuses the textarea.
test("Label click focuses labeled textarea", async ({ page }) => {
  await page.goto("/test_textarea");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test TextArea");

  await click_label_expect_focus(
    page,
    /A second box for text/,
    page.locator("#labeled-textarea"),
  );
});

// Check that clicking a standalone select's label focuses the select.
test("Label click focuses labeled select", async ({ page }) => {
  await page.goto("/test_select");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Select");

  await click_label_expect_focus(
    page,
    /Labeled select/,
    page.locator("#labeled-sel"),
  );
});

// Check that clicking the label of a <FormInput>-wrapped text input focuses the input.
test("Label click focuses FormInput-wrapped text input", async ({ page }) => {
  await page.goto("/forms");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Forms");

  await click_label_expect_focus(
    page,
    /Full name/,
    page.locator('input[name="fullname"]'),
  );
});

// Check that clicking the label of a <FormInput>-wrapped password input focuses the input.
test("Label click focuses FormInput-wrapped password input", async ({
  page,
}) => {
  await page.goto("/forms");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Forms");

  await click_label_expect_focus(
    page,
    /Password/,
    page.locator('input[name="password"]'),
  );
});

// Check that clicking the label of a <FormInput>-wrapped textarea focuses the textarea.
test("Label click focuses FormInput-wrapped textarea", async ({ page }) => {
  await page.goto("/forms");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Forms");

  await click_label_expect_focus(
    page,
    /Notes/,
    page.locator('textarea[name="notes"]'),
  );
});

// Check that clicking the label of a <FormInput>-wrapped maybe-select focuses the select.
test("Label click focuses FormInput-wrapped maybe-select", async ({ page }) => {
  await page.goto("/forms");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Forms");

  await click_label_expect_focus(
    page,
    /Favorite Element/,
    page.locator('select[name="favorite_element"]'),
  );
});

// Check that clicking the label of a <FormInput>-wrapped select focuses the select.
test("Label click focuses FormInput-wrapped select", async ({ page }) => {
  await page.goto("/forms");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Forms");

  await click_label_expect_focus(
    page,
    /An Element/,
    page.locator('select[name="an_element"]'),
  );
});

// Check that clicking the label of a <FormInput>-wrapped date-picker focuses the input.
test("Label click focuses FormInput-wrapped date picker", async ({ page }) => {
  await page.goto("/forms");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Forms");

  await click_label_expect_focus(
    page,
    /End date/,
    page.locator('input[name="end-date"]'),
  );
});


// Check that clicking the label of a <FormInput>-wrapped upload focuses it.
test("Label click focuses FormInput-wrapped upload", async ({ page }) => {
  await page.goto("/forms");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Forms");

  await click_label_expect_focus(
    page,
    /Wrapped upload/,
    page.locator('input[name="wrapped_upload"]'),
  );
});

// Check that clicking the label of a <FormInput>-wrapped checkbox toggles it.
test("Label click toggles FormInput-wrapped checkbox", async ({ page }) => {
  await page.goto("/forms");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Forms");

  await click_label_expect_toggle(
    page,
    /Wrapped checkbox/,
    page.locator('input[name="wrapped_checkbox"]'),
  );
});

// Check that clicking the label of a <FormInput>-wrapped toggle toggles it.
test("Label click toggles FormInput-wrapped toggle", async ({ page }) => {
  await page.goto("/forms");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Forms");

  await click_label_expect_toggle(
    page,
    /Wrapped toggle/,
    page.locator('input[name="wrapped_toggle"]'),
  );
});
