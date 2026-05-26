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

// Turns rgb(r, g, b) or #RRGGBB into their perceived brightness value (black=0 -> white=255).
function getBrightness(hexColor: string) {
  console.log(hexColor);
  if (hexColor.startsWith("rgb(")) {
    let colors = hexColor
      .replace("rgb(", "")
      .replace(")", "")
      .split(",")
      .map((s) => s.trim());

    const r = parseInt(colors[0]);
    const g = parseInt(colors[1]);
    const b = parseInt(colors[2]);

    return (r * 299 + g * 587 + b * 114) / 1000;
  } else if (hexColor.startsWith("#")) {
    // Remove hash if present
    const hex = hexColor.replace("#", "");

    // Extract RGB components
    const r = parseInt(hex.substring(0, 2), 16);
    const g = parseInt(hex.substring(2, 4), 16);
    const b = parseInt(hex.substring(4, 6), 16);

    return (r * 299 + g * 587 + b * 114) / 1000;
  } else {
    return undefined;
  }
}

/// Tests basic display and functionality of the toast.
test("test theme_selector", async ({ page }) => {
  await page.goto("/");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Leptodon");

  let theme_sel_1 = page.locator("#theme-selector-1");
  let theme_sel_2 = page.locator("#theme-selector-2");
  let html = page.locator("html");
  let meta = page.locator("meta[name='color-scheme']");

  await expect(theme_sel_1).toHaveCount(1);
  await expect(theme_sel_2).toHaveCount(1);

  async function checkTheme(
    theme: "dark" | "light",
    is_system: boolean = false,
  ) {
    await expect(html).toHaveClass(theme);
    await expect(meta).toHaveAttribute(
      "content",
      is_system ? (theme == "dark" ? "dark light" : "light dark") : theme,
    );

    // Currently undefined for light theme.
    // Browser will draw light/dark background based on the meta color-scheme when none is drawn in front.
    let brightness = getBrightness(
      await page.evaluate(
        () => window.getComputedStyle(document.documentElement).backgroundColor,
      ),
    );

    if (theme == "light") {
      expect(brightness || 255).toBeGreaterThan(125);
    } else {
      expect(brightness || 0).toBeLessThan(125);
    }
  }

  // Check dark (default theme)
  await expect(theme_sel_1).toHaveValue("dark");
  await expect(theme_sel_2).toHaveValue("dark");
  await checkTheme("dark");

  // Swap to light
  await theme_sel_1.selectOption("light");

  // Check light
  await expect(theme_sel_1).toHaveValue("light");
  await expect(theme_sel_2).toHaveValue("light");
  await checkTheme("light");

  // Reload
  page.reload({ waitUntil: "networkidle" });
  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Leptodon");

  // Check light
  await expect(theme_sel_1).toHaveValue("light");
  await expect(theme_sel_2).toHaveValue("light");
  await checkTheme("light");

  // Swap to system
  // sel_2 doesn't affect sel_1 here, but it does affect sel_1 in manual browser usage.
  await theme_sel_1.selectOption("follow_system");
  await theme_sel_2.selectOption("follow_system");

  // Check system
  await expect(theme_sel_1).toHaveValue("follow_system");
  await expect(theme_sel_2).toHaveValue("follow_system");

  // Swap system preference to dark.
  await page.emulateMedia({ colorScheme: "dark" });

  // Check dark
  await checkTheme("dark", true);

  // Swap system preference to light.
  await page.emulateMedia({ colorScheme: "light" });

  // Check light
  await checkTheme("light", true);

  page.reload();
  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Leptodon");

  // Check light
  await checkTheme("light", true);
});
