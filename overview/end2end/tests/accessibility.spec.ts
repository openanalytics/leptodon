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
// import { test, expect } from "@playwright/test";
// import AxeBuilder from '@axe-core/playwright'; // 1

// test.describe('homepage', () => { // 2
//   test('should not have any automatically detectable accessibility issues', async ({ page }) => {
//     await page.goto('http://localhost:3000/'); // 3
//     await page.waitForLoadState("networkidle");
//     await expect(page).toHaveTitle("Leptodon");

//     const accessibilityScanResults = await new AxeBuilder({ page }).analyze(); // 4

//     expect(accessibilityScanResults.violations).toEqual([]); // 5
//   });
// });
