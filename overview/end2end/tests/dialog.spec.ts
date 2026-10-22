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

test.describe("Leptos Dialog Component", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/test_dialog");
    await page.waitForLoadState("networkidle");
    await expect(page).toHaveTitle("Test Dialog");
  });

  test("Dialog is initially hidden", async ({ page }) => {
    const dialogContent = page.getByText("test-content-in-dialog");
    await expect(dialogContent).not.toBeVisible();
  });

  test("Dialog opens and shows correct values", async ({ page }) => {
    // Click the button to show the dialog
    await page.getByRole("button", { name: "Show Dialog" }).click();

    // Verify dialog title and content are set
    await expect(page.getByText("Dialog testing title")).toBeVisible();
    await expect(page.getByText("test-content-in-dialog")).toBeVisible();

    // Verify primary and secondary action buttons/texts are set
    await expect(
      page.getByRole("button", { name: "PrimaryText" }),
    ).toBeVisible();
    await expect(
      page.getByRole("button", { name: "SecondaryText" }),
    ).toBeVisible();
  });

  test("Close via secondary", async ({ page }) => {
    await page.getByRole("button", { name: "Show Dialog" }).click();

    const consoleMessages: string[] = [];

    // Listen to all console events
    page.on("console", (msg) => {
      consoleMessages.push(msg.text());
    });

    // Click the secondary button
    await page.getByRole("button", { name: "SecondaryText" }).click();

    expect(consoleMessages).toContain("on_click_secondary ran");
    expect(consoleMessages).toContain("on_close_or_secondary ran");
  });

  test("Close via backdrop", async ({ page }) => {
    await page.getByRole("button", { name: "Show Dialog" }).click();

    const consoleMessages: string[] = [];

    // Listen to all console events
    page.on("console", (msg) => {
      consoleMessages.push(msg.text());
    });

    await page.getByTestId("dialog-backdrop").click({
      position: { x: 0, y: 0 },
    });

    expect(consoleMessages).toContain("on_close ran");
    expect(consoleMessages).toContain("on_close_or_secondary ran");
  });

  test("Close via title button", async ({ page }) => {
    await page.getByRole("button", { name: "Show Dialog" }).click();

    const consoleMessages: string[] = [];

    // Listen to all console events
    page.on("console", (msg) => {
      consoleMessages.push(msg.text());
    });


    let close_button = page.getByRole("dialog").getByRole("button").first();
    await close_button.isVisible();

    await close_button.click();

    await expect.poll(() => consoleMessages).toContain("on_close ran");
    await expect
      .poll(() => consoleMessages)
      .toContain("on_close_or_secondary ran");
  });
});
