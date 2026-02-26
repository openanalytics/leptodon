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

// Checks that all selected files are listed,
test("Upload input", async ({ page }) => {
  await page.goto("http://localhost:3000/test_upload");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Test Upload");

  let file_upload = page.locator("#file-upload");

  await file_upload.click();
  await file_upload.setInputFiles([
    './tests/test_files/s2.png',
    './tests/test_files/s3.png',
  ]);
  await page.getByText('s2').click();
  await page.getByText('s3').click();
});
