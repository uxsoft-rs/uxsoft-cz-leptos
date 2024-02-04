import { test, expect } from "@playwright/test";

test("homepage has title and links to intro page", async ({ page }) => {
  await page.goto("https://uxsoft.cz/");

  await expect(page).toHaveTitle("uxsoft - Welcome");

  await expect(page.locator("h1")).toHaveText("Welcome");
});
