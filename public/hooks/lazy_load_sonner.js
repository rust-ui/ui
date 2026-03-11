/**
 * Lazy Sonner Loader
 * Loads Sonner toast library only when needed (on-demand)
 * Improves initial page load performance by deferring non-critical scripts
 */

(() => {
  // Prevent multiple initializations
  if (window.LazySonner) {
    return;
  }

  class LazySonner {
    constructor() {
      this.loaded = false;
      this.loading = false;
      this.loadPromise = null;
      this.pendingToasts = [];
    }

    /**
     * Load Sonner script dynamically
     * @returns {Promise} Resolves when script is loaded
     */
    load() {
      // Return existing promise if already loading
      if (this.loadPromise) {
        return this.loadPromise;
      }

      // Return resolved promise if already loaded
      if (this.loaded) {
        return Promise.resolve();
      }

      this.loading = true;

      this.loadPromise = new Promise((resolve, reject) => {
        const script = document.createElement("script");
        script.type = "module";
        script.async = true;
        script.src = "/coming_soon/sonner.js";

        script.onload = () => {
          this.loaded = true;
          this.loading = false;
          console.debug("Sonner loaded successfully");
          resolve();
        };

        script.onerror = () => {
          this.loading = false;
          this.loadPromise = null;
          console.error("Failed to load Sonner");
          reject(new Error("Failed to load Sonner"));
        };

        document.head.appendChild(script);
      });

      return this.loadPromise;
    }

    /**
     * Setup observers to detect when Sonner triggers appear
     */
    observeTriggers() {
      // Check for existing triggers on page
      const checkAndLoad = () => {
        const triggers = document.querySelectorAll('[data-name="SonnerTrigger"]');
        const wrapper = document.querySelector('[data-name="SonnerList"]');

        if ((triggers.length > 0 || wrapper) && !this.loaded && !this.loading) {
          console.debug("Sonner elements detected - loading script");
          this.load();
        }
      };

      // Initial check
      checkAndLoad();

      // Watch for dynamically added triggers (SPA navigation)
      const observer = new MutationObserver((mutations) => {
        mutations.forEach((mutation) => {
          if (mutation.type === "childList") {
            mutation.addedNodes.forEach((node) => {
              if (node.nodeType === Node.ELEMENT_NODE) {
                const hasTrigger = node?.querySelector?.('[data-name="SonnerTrigger"]');
                const hasWrapper = node?.querySelector?.('[data-name="SonnerList"]');
                const isTrigger =
                  node.getAttribute && node.getAttribute("data-name") === "SonnerTrigger";
                const isWrapper =
                  node.getAttribute && node.getAttribute("data-name") === "SonnerList";

                if (hasTrigger || hasWrapper || isTrigger || isWrapper) {
                  if (!this.loaded && !this.loading) {
                    console.debug("Sonner elements detected via observer - loading script");
                    this.load();
                  }
                }
              }
            });
          }
        });
      });

      // Start observing
      observer.observe(document.body, {
        childList: true,
        subtree: true,
      });
    }

    /**
     * Preload Sonner on user interaction (optional optimization)
     */
    preloadOnInteraction() {
      const events = ["mouseover", "touchstart", "keydown"];
      const preload = () => {
        if (!this.loaded && !this.loading) {
          this.load();
        }
        // Remove listeners after first interaction
        events.forEach((event) => {
          document.removeEventListener(event, preload, { once: true });
        });
      };

      // Add listeners for first interaction
      events.forEach((event) => {
        document.addEventListener(event, preload, { once: true });
      });
    }
  }

  // Export as singleton
  const lazySonner = new LazySonner();
  window.LazySonner = lazySonner;

  // Start observing for Sonner elements
  lazySonner.observeTriggers();

  // Optional: Preload on first user interaction for better UX
  // Comment out if you want strictly on-demand loading
  lazySonner.preloadOnInteraction();
})();
