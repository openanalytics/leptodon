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

test("Popover functionality", async ({ page }) => {
  await page.goto("http://localhost:3000/test_popover");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Popover");

  const r1top = page.locator("#row1-top");
  const r1topPopover = page.locator("#row1-top-popover");
  const r1bot = page.locator("#row1-bot");
  const r1botPopover = page.locator("#row1-bot-popover");
  const r1left = page.locator("#row1-left");
  const r1leftPopover = page.locator("#row1-left-popover");
  const r1right = page.locator("#row1-right");
  const r1rightPopover = page.locator("#row1-right-popover");

  // R1 LEFT
  await r1left.hover();
  await expect(r1leftPopover).toBeVisible();

  const triggerBox = await r1left.boundingBox();
  const popoverBox = await r1leftPopover.boundingBox();
  if (!triggerBox || !popoverBox) {
    throw new Error("Could not get bounding boxes");
  }

  // Check that right side of popover aligns with left side of trigger.
  const x_gap = Math.abs(popoverBox.x + popoverBox.width - triggerBox.x);

  // Vertical centers should align
  const y_shift = Math.abs(
    (popoverBox.y + popoverBox.height / 2) -
      (triggerBox.y + triggerBox.height / 2),
  );

  expect(x_gap).toBeLessThanOrEqual(20);
  expect(y_shift).toBeLessThanOrEqual(10);

  // R1TOP
  //

});
