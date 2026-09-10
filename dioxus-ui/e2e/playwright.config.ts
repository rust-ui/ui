import { defineConfig, devices } from "@playwright/test";

// Support multiple ways to configure the server URL:
// 1. BASE_URL - full URL (e.g., "http://localhost:8080")
// 2. PORT - port number (e.g., "8080")
const getBaseUrl = () => {
  if (process.env.BASE_URL) return process.env.BASE_URL;
  if (process.env.PORT) return `http://127.0.0.1:${process.env.PORT}`;
  return "http://127.0.0.1:8080";
};

const BASE_URL = getBaseUrl();

export default defineConfig({
  testDir: "./tests",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : 8,
  reporter: [["html", { open: "never" }], ["list"]],

  timeout: 30000,
  expect: {
    timeout: 5000,
  },

  use: {
    baseURL: BASE_URL,
    viewport: { width: 1280, height: 720 },
    screenshot: "only-on-failure",
    video: "retain-on-failure",
    trace: "on-first-retry",
  },

  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
    // {
    //   name: "firefox",
    //   use: { ...devices["Desktop Firefox"] },
    // },
    // {
    //   name: "webkit",
    //   use: { ...devices["Desktop Safari"] },
    // },
  ],

  // webServer: {
  //   command: "dx serve",
  //   url: BASE_URL,
  //   reuseExistingServer: !process.env.CI,
  //   timeout: 120000,
  // },
});
