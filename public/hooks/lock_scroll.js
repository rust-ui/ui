/**
 * Scroll Lock Utility
 * Handles locking and unlocking scroll for both window and all scrollable containers
 * Similar to react-remove-scroll but in vanilla JavaScript
 */

(() => {
  // Prevent multiple initializations
  if (window.ScrollLock) {
    return;
  }

  class ScrollLock {
    constructor() {
      this.locked = false;
      this.scrollableElements = [];
      this.scrollPositions = new Map();
      this.originalStyles = new Map();
      this.fixedElements = [];
    }

    /**
     * Find all scrollable elements in the DOM (optimized)
     * Uses more targeted selectors instead of querying all elements
     */
    findScrollableElements() {
      const scrollables = [];

      // More targeted query - only look for elements with overflow properties
      const candidates = document.querySelectorAll(
        '[style*="overflow"], [class*="overflow"], [class*="scroll"], main, aside, section, div',
      );

      // Batch all style reads first to minimize reflows
      const elementsToCheck = [];
      for (const el of candidates) {
        // Skip the element itself or if it's inside these containers
        const dataName = el.getAttribute("data-name");
        const isExcludedElement =
          dataName === "ScrollArea" ||
          dataName === "CommandList" ||
          dataName === "SelectContent" ||
          dataName === "MultiSelectContent" ||
          dataName === "DropdownMenuContent" ||
          dataName === "ContextMenuContent";

        if (
          el !== document.body &&
          el !== document.documentElement &&
          !isExcludedElement &&
          !el.closest('[data-name="ScrollArea"]') &&
          !el.closest('[data-name="CommandList"]') &&
          !el.closest('[data-name="SelectContent"]') &&
          !el.closest('[data-name="MultiSelectContent"]') &&
          !el.closest('[data-name="DropdownMenuContent"]') &&
          !el.closest('[data-name="ContextMenuContent"]')
        ) {
          elementsToCheck.push(el);
        }
      }

      // Now batch read all computed styles and dimensions
      elementsToCheck.forEach((el) => {
        const style = window.getComputedStyle(el);
        const hasOverflow =
          style.overflow === "auto" ||
          style.overflow === "scroll" ||
          style.overflowY === "auto" ||
          style.overflowY === "scroll";

        // Only check scrollHeight if overflow is set
        if (hasOverflow && el.scrollHeight > el.clientHeight) {
          scrollables.push(el);
        }
      });

      return scrollables;
    }

    /**
     * Lock scrolling on all scrollable elements (optimized)
     * Batches all DOM reads before DOM writes to prevent forced reflows
     */
    lock() {
      if (this.locked) return;

      this.locked = true;

      // Find all scrollable elements
      this.scrollableElements = this.findScrollableElements();

      // ===== BATCH 1: READ PHASE - Read all layout properties first =====
      const windowScrollY = window.scrollY;
      const scrollbarWidth = window.innerWidth - document.body.clientWidth;

      // Store window scroll position
      this.scrollPositions.set("window", windowScrollY);

      // Store original body styles
      this.originalStyles.set("body", {
        position: document.body.style.position,
        top: document.body.style.top,
        width: document.body.style.width,
        overflow: document.body.style.overflow,
        paddingRight: document.body.style.paddingRight,
      });

      // Read all fixed-position elements and their padding (only if we have scrollbar)
      if (scrollbarWidth > 0) {
        // Use more targeted query for fixed elements
        const fixedCandidates = document.querySelectorAll(
          '[style*="fixed"], [class*="fixed"], header, nav, aside, [role="dialog"], [role="alertdialog"]',
        );

        this.fixedElements = Array.from(fixedCandidates).filter((el) => {
          const style = window.getComputedStyle(el);
          return (
            style.position === "fixed" &&
            !el.closest('[data-name="DropdownMenuContent"]') &&
            !el.closest('[data-name="MultiSelectContent"]') &&
            !el.closest('[data-name="ContextMenuContent"]')
          );
        });

        // Batch read all padding values
        this.fixedElements.forEach((el) => {
          const computedStyle = window.getComputedStyle(el);
          const currentPadding = Number.parseInt(computedStyle.paddingRight, 10) || 0;

          this.originalStyles.set(el, {
            paddingRight: el.style.paddingRight,
            computedPadding: currentPadding,
          });
        });
      }

      // Read scrollable elements info
      const scrollableInfo = this.scrollableElements.map((el) => {
        const scrollTop = el.scrollTop;
        const elementScrollbarWidth = el.offsetWidth - el.clientWidth;
        const computedStyle = window.getComputedStyle(el);
        const currentPadding = Number.parseInt(computedStyle.paddingRight, 10) || 0;

        this.scrollPositions.set(el, scrollTop);
        this.originalStyles.set(el, {
          overflow: el.style.overflow,
          overflowY: el.style.overflowY,
          paddingRight: el.style.paddingRight,
        });

        return { el, elementScrollbarWidth, currentPadding };
      });

      // ===== BATCH 2: WRITE PHASE - Apply all styles at once =====

      // Apply body lock
      document.body.style.position = "fixed";
      document.body.style.top = `-${windowScrollY}px`;
      document.body.style.width = "100%";
      document.body.style.overflow = "hidden";

      if (scrollbarWidth > 0) {
        document.body.style.paddingRight = `${scrollbarWidth}px`;

        // Apply padding compensation to fixed elements
        this.fixedElements.forEach((el) => {
          const stored = this.originalStyles.get(el);
          if (stored) {
            el.style.paddingRight = `${stored.computedPadding + scrollbarWidth}px`;
          }
        });
      }

      // Lock all scrollable containers
      scrollableInfo.forEach(({ el, elementScrollbarWidth, currentPadding }) => {
        el.style.overflow = "hidden";

        if (elementScrollbarWidth > 0) {
          el.style.paddingRight = `${currentPadding + elementScrollbarWidth}px`;
        }
      });
    }

    /**
     * Unlock scrolling on all elements (optimized)
     * @param {number} delay - Delay in milliseconds before unlocking (for animations)
     */
    unlock(delay = 0) {
      if (!this.locked) return;

      const performUnlock = () => {
        // Restore body scroll
        const bodyStyles = this.originalStyles.get("body");
        if (bodyStyles) {
          document.body.style.position = bodyStyles.position;
          document.body.style.top = bodyStyles.top;
          document.body.style.width = bodyStyles.width;
          document.body.style.overflow = bodyStyles.overflow;
          document.body.style.paddingRight = bodyStyles.paddingRight;
        }

        // Restore window scroll position
        const windowScrollY = this.scrollPositions.get("window") || 0;
        window.scrollTo(0, windowScrollY);

        // Restore all scrollable containers
        this.scrollableElements.forEach((el) => {
          const originalStyles = this.originalStyles.get(el);
          if (originalStyles) {
            el.style.overflow = originalStyles.overflow;
            el.style.overflowY = originalStyles.overflowY;
            el.style.paddingRight = originalStyles.paddingRight;
          }

          // Restore scroll position
          const scrollPosition = this.scrollPositions.get(el) || 0;
          el.scrollTop = scrollPosition;
        });

        // Restore fixed-position elements padding
        this.fixedElements.forEach((el) => {
          const styles = this.originalStyles.get(el);
          if (styles && styles.paddingRight !== undefined) {
            el.style.paddingRight = styles.paddingRight;
          }
        });

        // Clear storage
        this.scrollableElements = [];
        this.fixedElements = [];
        this.scrollPositions.clear();
        this.originalStyles.clear();
        this.locked = false;
      };

      if (delay > 0) {
        setTimeout(performUnlock, delay);
      } else {
        performUnlock();
      }
    }

    /**
     * Check if scrolling is currently locked
     */
    isLocked() {
      return this.locked;
    }
  }

  // Export as singleton
  window.ScrollLock = new ScrollLock();
})();
