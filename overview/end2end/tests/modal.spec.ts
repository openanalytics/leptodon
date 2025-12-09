import { test, expect } from "@playwright/test";

// Check modal opening and closing with keyboard.
test("Modal opens and closes.", async ({ page }) => {
  await page.goto("http://localhost:3000/");
  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Leptos components");

  // Assert closed
  await expect(
    page.getByRole("dialog", { name: "Example modal" }),
  ).toBeHidden();
  // Try opening modal
  await page.getByRole("button", { name: "Toggle Modal" }).click();
  // Assert Open
  await expect(
    page.getByRole("dialog", { name: "Example modal" }),
  ).toBeVisible();

  // Focus tab-transfer from toggle-modal-btn -?> first button (close button).
  await page.getByRole("button", { name: "Toggle Modal" }).press("Tab");
  await expect(
    page
      .getByLabel("Example modal")
      .getByRole("button")
      .filter({ hasText: /^$/ }),
  ).toBeFocused();

  // Focus transfer from first button -> next button (Dispose modal button).
  await page
    .getByLabel("Example modal")
    .getByRole("button")
    .filter({ hasText: /^$/ })
    .press("Tab");
  await expect(
    page.getByRole("button", { name: "Dispose modal" }),
  ).toBeFocused();

  // Try closing
  await page.getByRole("button", { name: "Dispose modal" }).press("Space");
  await expect(
    page.getByRole("dialog", { name: "Example modal" }),
  ).toBeHidden();
});

// Check modal, focus looping => modality.
test("Modal focus looping.", async ({ page }) => {
  await page.goto("http://localhost:3000/");
  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Leptos components");

  let closeButton = page
    .getByLabel("Example modal")
    .getByRole("button")
    .filter({ hasText: /^$/ });

  // Try opening modal
  await page.getByRole("button", { name: "Toggle Modal" }).click();

  // Focus tab-transfer from toggle-modal-btn -?> first button (close button).
  await page.getByRole("button", { name: "Toggle Modal" }).press("Tab");
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
