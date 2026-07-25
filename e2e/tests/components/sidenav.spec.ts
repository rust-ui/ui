import { expect, test } from "@playwright/test";

const SIDENAV_ROUTE = "/view/sidenav01/docs/components/accordion";

test.describe("Sidenav keyboard shortcut", () => {
  test.beforeEach(async ({ page }) => {
    await page.route("**/app_components/sidenav.js*", (route) => route.abort());
    await page.goto(SIDENAV_ROUTE);
    await page.waitForLoadState("networkidle");
  });

  test("toggles with Control+B when the legacy JavaScript is unavailable", async ({
    page,
  }) => {
    const sidenav = page.locator('[data-name="Sidenav"]').first();

    await expect(sidenav).toHaveAttribute("data-state", "Expanded");
    await page.keyboard.press("Control+b");
    await expect(sidenav).toHaveAttribute("data-state", "Collapsed");
  });

  test("toggles with Meta+B", async ({ page }) => {
    const sidenav = page.locator('[data-name="Sidenav"]').first();

    await expect(sidenav).toHaveAttribute("data-state", "Expanded");
    await page.keyboard.press("Meta+b");
    await expect(sidenav).toHaveAttribute("data-state", "Collapsed");
  });

  test("does not toggle while an input is focused", async ({ page }) => {
    const sidenav = page.locator('[data-name="Sidenav"]').first();
    const search = page
      .locator('[data-name="SidenavWrapper"] input#search')
      .first();

    await search.focus();
    await page.keyboard.press("Control+b");
    await expect(sidenav).toHaveAttribute("data-state", "Expanded");
  });

  test("does not toggle for a focused descendant of a contenteditable region", async ({
    page,
  }) => {
    const sidenav = page.locator('[data-name="Sidenav"]').first();

    await page.evaluate(() => {
      const editor = document.createElement("div");
      editor.contentEditable = "true";
      const child = document.createElement("span");
      child.tabIndex = 0;
      child.textContent = "Editable child";
      editor.appendChild(child);
      document.body.appendChild(editor);
      child.focus();
    });

    await page.keyboard.press("Control+b");
    await expect(sidenav).toHaveAttribute("data-state", "Expanded");
  });
});
