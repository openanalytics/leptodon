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

// Check modal opening and closing with keyboard.
test("Modal opens and closes.", async ({ page }) => {
  await page.goto("http://localhost:3000/");
  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Leptodon");

  const toggle_button = page.getByRole("button", { name: "Toggle Modal" });
  const example_modal = page.getByRole("dialog", { name: "Example modal" });
  // Assert closed
  await expect(example_modal).toBeHidden();
  // Try opening modal
  await toggle_button.click();
  // Assert Open
  await expect(example_modal).toBeVisible();

  // Focus tab-transfer from toggle-modal-btn -?> first button (close button).
  await toggle_button.press("Tab");
  await expect(
    example_modal.getByRole("button").filter({ hasText: /^$/ }),
  ).toBeFocused();

  // Focus transfer from first button -> next button (Dispose modal button).
  await example_modal
    .getByRole("button")
    .filter({ hasText: /^$/ })
    .press("Tab");
  await expect(
    example_modal.getByRole("button", { name: "Dispose modal" }),
  ).toBeFocused();

  // Try closing
  await example_modal
    .getByRole("button", { name: "Dispose modal" })
    .press("Space");
  await expect(example_modal).toBeHidden();

  // Open again
  await toggle_button.click();
  await expect(example_modal).toBeVisible(); // This line is needed otherwise the next escape press happens to quickly for chrome.
  await example_modal.click(); // Click to make sure its focused on chrome during tests.

  // Close with Escape
  await page.keyboard.press("Escape");
  await expect(example_modal).toBeHidden();
});

// Check modal, focus looping => modality.
test("Modal focus looping.", async ({ page }) => {
  await page.goto("http://localhost:3000/");
  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Leptodon");

  const toggle_button = page.getByRole("button", { name: "Toggle Modal" });
  let closeButton = page
    .getByLabel("Example modal")
    .getByRole("button")
    .filter({ hasText: /^$/ });

  // Try opening modal
  await toggle_button.click();

  // Focus tab-transfer from toggle-modal-btn -?> first button (close button).
  await toggle_button.press("Tab");
  await expect(closeButton).toBeFocused();

  // Focus transfer from first button -> next button (Dispose modal button).
  await closeButton.press("Tab");
  await expect(
    page.getByRole("button", { name: "Dispose modal" }),
  ).toBeFocused();

  // Focus transfer from last button (dispose-button) -> first button (Close modal button).
  await page.getByRole("button", { name: "Dispose modal" }).press("Tab");
  await expect(closeButton).toBeFocused();
});
