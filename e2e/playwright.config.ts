import { defineConfig, devices } from "@playwright/test";

// Support multiple ways to configure the server URL:
// 1. BASE_URL - full URL (e.g., "http://localhost:8080")
// 2. PORT - port number (e.g., "8080")
// 3. LEPTOS_SITE_ADDR - Leptos format (e.g., "127.0.0.1:8080")
const getBaseUrl = () => {
  if (process.env.BASE_URL) return process.env.BASE_URL;
  if (process.env.PORT) return `http://127.0.0.1:${process.env.PORT}`;
  if (process.env.LEPTOS_SITE_ADDR) return `http://${process.env.LEPTOS_SITE_ADDR}`;
  return "http://127.0.0.1:3000";
};

const BASE_URL = getBaseUrl();

/**
 * See https://playwright.dev/docs/test-configuration.
 */
export default defineConfig({
  testDir: "./tests",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : 8, // 8 parallel workers locally
  reporter: [["html", { open: "never" }], ["list"]],

  // Timeouts
  timeout: 30000,
  expect: {
    timeout: 5000,
  },

  use: {
    baseURL: BASE_URL,

    // Viewport
    viewport: { width: 1280, height: 720 },

    // Artifacts
    screenshot: "only-on-failure",
    video: "retain-on-failure",
    trace: "on-first-retry",
  },

  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
    // Uncomment to test in other browsers
    // {
    //   name: "firefox",
    //   use: { ...devices["Desktop Firefox"] },
    // },
    // {
    //   name: "webkit",
    //   use: { ...devices["Desktop Safari"] },
    // },
  ],

  // Uncomment to auto-start the dev server
  // webServer: {
  //   command: "cargo leptos watch",
  //   url: BASE_URL,
  //   reuseExistingServer: !process.env.CI,
  //   timeout: 120000,
  // },
});
