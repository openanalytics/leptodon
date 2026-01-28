import { test, expect } from "@playwright/test";

// Check that the radio buttons can be clicked on their label, surrounding div etc.
test("Radio button functionality", async ({ page }) => {
  await page.goto("http://localhost:3000/forms");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Forms");

  let radio_input = page.getByTestId("radio-input");
  await radio_input.getByRole("heading", { name: "*Radio Stations" }).click();

  // Click surrounding div.
  await radio_input.locator("div").filter({ hasText: "Radio1" }).click();
  await expect(
    radio_input.getByRole("radio", { name: "Radio1" }),
  ).toBeChecked();
  // Click label.
  await radio_input.getByText("Radio2").click();
  await expect(
    radio_input.getByRole("radio", { name: "Radio2" }),
  ).toBeChecked();
  // Click radio orb.
  await radio_input.getByRole("radio", { name: "Klara" }).check();
  await expect(radio_input.getByRole("radio", { name: "Klara" })).toBeChecked();
});
