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

// Accordion does its main task
test("Accordion opens and closes.", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Leptodon");

  let mainAccordionContent = page.getByText("Flowbite is an open - source");
  let subAccordionContent = page.getByText("Version 2.7.2 is available");
  await expect(mainAccordionContent).toBeHidden();
  await expect(subAccordionContent).toBeHidden();

  // Open main accordion
  await page.getByRole("button", { name: "What is Flowbite?" }).click();
  await expect(mainAccordionContent).toBeVisible();
  await expect(subAccordionContent).toBeHidden();

  // Open sub-acordion
  await page.getByRole("button", { name: "What about version 2.7.2?" }).click();
  await expect(mainAccordionContent).toBeVisible();
  await expect(subAccordionContent).toBeVisible();

  // Close sub-acordion
  await page.getByRole("button", { name: "What about version 2.7.2?" }).click();
  await expect(mainAccordionContent).toBeVisible();
  await expect(subAccordionContent).toBeHidden();

  // Close main accordion
  await page.getByRole("button", { name: "What is Flowbite?" }).click();
  await expect(mainAccordionContent).toBeHidden();
  await expect(subAccordionContent).toBeHidden();

  // Open main accordion
  await page.getByRole("button", { name: "What is Flowbite?" }).click();
  await expect(mainAccordionContent).toBeVisible();
  await expect(subAccordionContent).toBeHidden();

  // Open-sub accordion
  await page.getByRole("button", { name: "What about version 2.7.2?" }).click();
  await expect(mainAccordionContent).toBeVisible();
  await expect(subAccordionContent).toBeVisible();

  // Close main accordion
  await page.getByRole("button", { name: "What is Flowbite?" }).click();
  await expect(mainAccordionContent).toBeHidden();
  await expect(subAccordionContent).toBeHidden();

  // Open main accordion
  await page.getByRole("button", { name: "What is Flowbite?" }).click();
  await expect(mainAccordionContent).toBeVisible();
  await expect(subAccordionContent).toBeVisible();
});
