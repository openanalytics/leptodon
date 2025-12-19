import { test, expect } from "@playwright/test";

// Accordion does its main task
test("Accordion opens and closes.", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Leptos components");

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
