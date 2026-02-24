import { test, expect } from "@playwright/test";

// Main table features work.
test("Table shows data", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Leptodon");

  // Unsorted table for reference.
  // Detail Id	Name	Age	  Date      	Admin Editable
  // >      1	  John	32	  2025-10-10	  ✏️
  // >      2	  Jane	28	  2025-10-10	  ✏️
  // >      3	  Bob 	45	  2025-10-10	  ✏️
  // >      4	  Bob	  46	  2025-10-10	  ✏️
  // >      5	  Bob	  47	  2025-10-10	  ✏️
  // >      6	  Bob	  45	  2025-10-10    ✏️

  let person_table = page.getByTestId("person-table");
  // Find and click john
  await person_table.getByRole("cell", { name: "John" }).click();

  // Check detail view expansion
  await expect(
    person_table.getByRole("cell", { name: "De bouwer" }),
  ).toBeHidden();
  await person_table.locator(".person-row-5-col-Detail").click(); // Bob De Bouwer row.
  await expect(
    person_table.getByRole("cell", { name: "De bouwer" }),
  ).toBeVisible();

  // Have 6 rows
  await person_table.getByRole("cell", { name: "6", exact: true }).click();
  await person_table.getByRole("cell", { name: "5", exact: true }).click();
  await person_table.getByRole("cell", { name: "4", exact: true }).click();
  await person_table.getByRole("cell", { name: "3", exact: true }).click();
  await person_table.getByRole("cell", { name: "2", exact: true }).click();
  await person_table.getByRole("cell", { name: "1", exact: true }).click();

  // Have column-headers
  await expect(
    person_table.getByRole("columnheader", { name: "Date" }),
  ).toBeVisible();

  // Have footer
  await person_table.getByRole("cell", { name: "A Footer" }).click();

  // Have sorting
  await person_table.getByRole("columnheader", { name: "Age" }).click();
  await person_table.getByRole("columnheader", { name: "Age ▲" }).click();

  // Under the assumption the table is sorted now as Age ⏷ (descending)
  // Header = row 0
  // Placeholder = row 1
  // First content row = row 2
  await expect(
    person_table
      .locator("tr")
      .nth(2)
      .getByRole("cell", { name: "5", exact: true }), // Row 5 with age 47 should be first
  ).toBeVisible();
});

// Column dnd

test("Table column drag and drop", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await page.waitForLoadState("networkidle");
  await expect(page).toHaveTitle("Leptodon");

  let person_table = page.getByTestId("person-table");
  const headers = person_table.getByRole("columnheader");
  // Detail Id	Name	Age	  Date      	Admin Editable
  await expect(headers.nth(0)).toHaveText("Detail");
  await expect(headers.nth(1)).toHaveText("Id");
  await expect(headers.nth(2)).toHaveText("Name");
  await expect(headers.nth(3)).toHaveText("Age");
  await expect(headers.nth(4)).toHaveText("Date");
  await expect(headers.nth(5)).toHaveText("Admin Editable");

  await person_table.getByRole("columnheader", { name: "Age" }).hover();
  await page.mouse.down();
  const id = await person_table
    .getByRole("columnheader", { name: "Id" })
    .boundingBox();
  if (id == null) {
    expect(id != null);
    return;
  }
  await page.mouse.move(id.x + id.width - 1, id.y + id.height / 2);
  await page.mouse.move(id.x + id.width - 2, id.y + id.height / 2);
  await page.mouse.up();

  await expect(headers.nth(0)).toHaveText("Detail");
  await expect(headers.nth(1)).toHaveText("Id");
  await expect(headers.nth(2)).toHaveText("Age");
  await expect(headers.nth(3)).toHaveText("Name");
  await expect(headers.nth(4)).toHaveText("Date");
  await expect(headers.nth(5)).toHaveText("Admin Editable");
});
