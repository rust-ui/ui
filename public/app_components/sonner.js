// Toast props
const TOAST_PROPS = {
  MAX_TOASTS: "--max-toasts",
};

const TOAST_DATA_NAMES = {
  LIST: "SonnerList",
  ITEM: "SonnerItem",
  TRIGGER: "SonnerTrigger",
};

// Timer start mode
const TIMER_START = {
  ON_ANIMATION_END: "on_animation_end",
  IMMEDIATELY: "immediately",
};

// Swipe configuration
const SWIPE_THRESHOLD = 45; // pixels
const SWIPE_VELOCITY_THRESHOLD = 0.11; // pixels per millisecond

// Visible toasts amount (Sonner default)
const VISIBLE_TOASTS_AMOUNT = 3;

// Default toast container styles (CSS variables)
const DEFAULT_TOAST_STYLES = {
  "--max-toasts": "5",
  "--dismiss-delay": "5000ms",
  "--enter-duration": "300ms",
  "--exit-duration": "300ms",
  "--stack-duration": "300ms",
  "--stack-spacing": "20px",
  "--expand-spacing": "110px",
  "--gap": "15px",
  "--scale-factor": "0.05",
  "--transition-easing": "ease-out",
  "--stack-easing": "ease-in-out",
};

// Toast type icons (SVG)
const TOAST_ICONS = {
  success: `<svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
    <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
  </svg>`,
  error: `<svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/>
  </svg>`,
  warning: `<svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
    <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
  </svg>`,
  info: `<svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
    <path stroke-linecap="round" stroke-linejoin="round" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
  </svg>`,
  loading: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
    <path stroke-linecap="round" stroke-linejoin="round" d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
  </svg>`,
};

// Variant styling classes (Tailwind)
const VARIANT_CLASSES = {
  Default: "bg-background text-foreground border-border",
  Success: "bg-success-light text-success-dark border-success",
  Error: "bg-destructive-light text-destructive-dark border-destructive",
  Warning: "bg-warning-light text-warning-dark border-warning",
  Info: "bg-info-light text-info-dark border-info",
  Loading: "bg-background text-foreground border-border",
};

// Action button variant classes
const ACTION_BUTTON_CLASSES = {
  Success: "!bg-success !text-white hover:!bg-success/90 !border-success/90",
  Error: "!bg-destructive !text-white hover:!bg-destructive/90 !border-destructive/90",
};

// Close button icon
const CLOSE_ICON = `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/>
</svg>`;

// Filled checkmark icon (for successful promise toasts)
const CHECK_FILLED_ICON = `<svg viewBox="0 0 24 24" fill="currentColor">
  <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
</svg>`;

// Filled error icon (for failed promise toasts)
const ERROR_FILLED_ICON = `<svg viewBox="0 0 24 24" fill="currentColor">
  <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/>
</svg>`;

/* ========================================================== */
/*                     ✨ TOAST STATE ✨                      */
/* ========================================================== */

// Toast state management - tracks toasts per position in arrays
const toastState = new Map();
let toastIdCounter = 0;

// Get or create toast array for a position
function getToastArray(toastsWrapper) {
  if (!toastState.has(toastsWrapper)) {
    toastState.set(toastsWrapper, []);
  }
  return toastState.get(toastsWrapper);
}

// Add toast to state array
function addToastToState(toastsWrapper, toastElement) {
  const toasts = getToastArray(toastsWrapper);
  const toastId = toastIdCounter++;

  toastElement._toastId = toastId;
  toasts.push({
    id: toastId,
    element: toastElement,
  });

  return toastId;
}

// Remove toast from state array
function removeToastFromState(toastsWrapper, toastElement) {
  const toasts = getToastArray(toastsWrapper);
  const index = toasts.findIndex((t) => t.element === toastElement);

  if (index !== -1) {
    toasts.splice(index, 1);
  }
}

/* ========================================================== */
/*                     ✨ UTILITY FUNCTIONS ✨                */
/* ========================================================== */

// Get CSS custom property value as integer (used 6+ times)
function getCSSValue(element, property) {
  return Number.parseInt(getComputedStyle(element).getPropertyValue(property), 10);
}

// Filter active toasts (not removed) (used 3 times)
function getActiveToasts(toasts) {
  return toasts.filter((toast) => toast.element.dataset.removed !== "true");
}

// Helper function to get toast content from trigger element
function getContentFromTrigger(trigger) {
  const title = trigger.dataset.toastTitle;
  const description = trigger.dataset.toastDescription;
  if (!title) return null;

  const content = { title };
  if (description) content.description = description;
  return content;
}

// Extract y-position (Top/Bottom) and x-position (Left/Center/Right) from position string
// e.g., "TopLeft" → { y: "Top", x: "Left" }, "BottomCenter" → { y: "Bottom", x: "Center" }
function parsePosition(position) {
  if (!position) return { y: "Bottom", x: "Right" }; // default

  const y = position.startsWith("Top") ? "Top" : "Bottom";

  let x = "Right"; // default
  if (position.includes("Left")) x = "Left";
  else if (position.includes("Center")) x = "Center";

  return { y, x };
}

/* ========================================================== */
/*                     ✨ INITIALIZATION ✨                   */
/* ========================================================== */

// Get toaster by position or return default (first one)
function getToasterByPosition(position) {
  if (position) {
    const toaster = document.querySelector(`[data-name="SonnerList"][data-position="${position}"]`);
    if (toaster) return toaster;
  }
  // Fallback to first toaster
  return document.querySelector('[data-name="SonnerList"]');
}

// Initialize a single toaster wrapper
function initializeToaster(toastsWrapper) {
  // Check if wrapper already initialized
  if (toastsWrapper.hasAttribute("data-sonner-initialized")) {
    return;
  }

  // Mark wrapper as initialized
  toastsWrapper.setAttribute("data-sonner-initialized", "true");

  // Apply default styles to toast container
  Object.entries(DEFAULT_TOAST_STYLES).forEach(([property, value]) => {
    toastsWrapper.style.setProperty(property, value);
  });

  // Setup expanded state tracking
  setupExpandedState(toastsWrapper);
}

// Initialize Sonner toast functionality for SPA-compatible operation
function initializeSonner() {
  const toastsWrappers = document.querySelectorAll('[data-name="SonnerList"]');

  // Early return if no toasters exist
  if (toastsWrappers.length === 0) {
    return;
  }

  console.debug("Initializing Sonner toast functionality");

  // Initialize all toasters
  toastsWrappers.forEach((wrapper) => {
    initializeToaster(wrapper);
  });
}

// Use event delegation for trigger clicks - this handles dynamically added triggers
// and avoids timing issues with Leptos hydration
let delegationSetup = false;
function setupTriggerDelegation() {
  if (delegationSetup) return;
  delegationSetup = true;

  document.addEventListener("click", (event) => {
    // Find if click target is a SonnerTrigger or inside one
    const trigger = event.target.closest('[data-name="SonnerTrigger"]');
    if (!trigger) return;

    const variant = trigger.dataset.variant || "Default";
    const content = getContentFromTrigger(trigger);
    const position = trigger.dataset.toastPosition;

    // Find the toaster - initialize it if not already done
    const toastsWrapper = getToasterByPosition(position);
    if (toastsWrapper) {
      // Ensure toaster is initialized
      initializeToaster(toastsWrapper);
      createNewToast(toastsWrapper, variant, content ? { content } : {});
    }
  });
}

// Initialize immediately if DOM is already loaded
initializeSonner();
setupTriggerDelegation();

// Set up MutationObserver to watch for Sonner toasters being added (for SPA navigation)
const observer = new MutationObserver((mutations) => {
  mutations.forEach((mutation) => {
    if (mutation.type === "childList") {
      mutation.addedNodes.forEach((node) => {
        // Check if added node contains Sonner toasters or is a toaster itself
        if (node.nodeType === Node.ELEMENT_NODE) {
          const hasToastWrapper = node?.querySelector?.('[data-name="SonnerList"]');
          const isToastWrapper =
            node.getAttribute && node.getAttribute("data-name") === "SonnerList";

          if (hasToastWrapper || isToastWrapper) {
            console.debug("Sonner toaster detected via MutationObserver - initializing");
            initializeSonner();
          }
        }
      });
    }
  });
});

// Start observing DOM changes
observer.observe(document.body, {
  childList: true,
  subtree: true,
});

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

// Get default swipe directions based on toast position
function getSwipeDirections(position) {
  // For now, support right and down for BottomRight position
  if (position === "BottomRight") {
    return ["right", "down"];
  }
  return ["right", "left", "up", "down"];
}

// Calculate dampening factor for wrong-direction swipes
function getDampening(delta) {
  const factor = Math.abs(delta) / 20;
  return 1 / (1.5 + factor);
}

// Toast element creation utility
function createToastElement(variant = "Default", options = {}) {
  const toastItem = document.createElement("li");
  const content = options.content || { title: "Notification" };
  // Use lowercase for icon lookup
  const icon = TOAST_ICONS[variant.toLowerCase()];

  toastItem.dataset.name = TOAST_DATA_NAMES.ITEM;
  toastItem.dataset.sonnerToast = "true";
  toastItem.dataset.variant = variant;

  // Apply variant-specific Tailwind classes
  // Note: positioning (top-0/bottom-0) is added later based on toaster position
  const variantClasses = VARIANT_CLASSES[variant] || VARIANT_CLASSES.Default;
  // Note: Don't use Tailwind's transform/translate classes here - we use CSS custom properties for animations
  toastItem.className = `p-5 shadow-lg border rounded-lg cursor-grab absolute w-full max-w-96 touch-none flex items-start gap-3 ${variantClasses}`;
  toastItem.dataset.mounted = "false";
  toastItem.dataset.entering = "true"; // Prevent stacking transforms during entry animation
  toastItem.dataset.expanded = "false";
  toastItem.dataset.visible = "true";
  toastItem.dataset.swiping = "false";
  toastItem.dataset.removed = "false";
  toastItem.dataset.swipeOut = "false";
  toastItem.dataset.front = "false";

  // Set initial inline CSS variables (will be updated by updateToastStyles)
  toastItem.style.setProperty("--index", "0");
  toastItem.style.setProperty("--toasts-before", "0");
  toastItem.style.setProperty("--z-index", "1");

  // Build HTML with optional icon
  const iconClasses =
    variant === "Loading"
      ? "flex items-center justify-center w-5 h-5 shrink-0 mr-3 [&>svg]:animate-spin"
      : "flex items-center justify-center w-5 h-5 shrink-0 mr-3";
  const iconHtml = icon ? `<div data-icon class="${iconClasses}">${icon}</div>` : "";
  const closeButtonHtml =
    variant !== "Loading"
      ? `<button data-close-button aria-label="Close toast" type="button" class="w-5 h-5 flex items-center justify-center border-none bg-transparent text-muted-foreground cursor-pointer transition-colors duration-150 shrink-0 p-0 ml-auto hover:text-foreground">${CLOSE_ICON}</button>`
      : "";

  // Build action buttons HTML
  const actionsHtml = options.actions?.length
    ? `<div data-toast-actions class="flex gap-2 mt-3">
        ${options.actions
          .map(
            (action, index) =>
              `<button data-action-button data-button="true" data-action-index="${index}" type="button" class="px-3 py-1.5 text-[13px] font-medium rounded-md border border-black/10 bg-black/5 cursor-pointer transition-all duration-150 hover:bg-black/10 hover:border-black/20">${action.label}</button>`,
          )
          .join("")}
      </div>`
    : "";

  // Build description HTML (optional)
  const descriptionHtml = content.description
    ? `<p class="text-sm leading-[1.5] opacity-90 mt-1">${content.description}</p>`
    : "";

  toastItem.innerHTML = `
    ${iconHtml}
    <div class="flex-1">
      <div class="flex items-center gap-2">
        <h3 class="font-semibold text-base leading-[1.4] flex-1">${content.title}</h3>
        ${closeButtonHtml}
      </div>
      ${descriptionHtml}
      ${actionsHtml}
    </div>
    <div data-duration-track class="absolute bottom-0 inset-x-0 h-[3px] bg-black/10 overflow-hidden rounded-b-lg">
      <div data-duration-progress class="h-full w-full bg-current opacity-30 origin-left"></div>
    </div>
  `;

  // Add close button click handler if not loading
  if (variant !== "Loading") {
    const closeButton = toastItem.querySelector("[data-close-button]");
    closeButton.addEventListener("click", (e) => {
      e.stopPropagation(); // Prevent triggering swipe events
      dismissToast(toastItem);
    });
  }

  // Add action button click handlers and apply variant-specific classes
  if (options.actions?.length) {
    const actionButtons = toastItem.querySelectorAll("[data-action-button]");
    const actionButtonClasses = ACTION_BUTTON_CLASSES[variant] || "";

    actionButtons.forEach((button, index) => {
      // Apply variant-specific action button styling
      if (actionButtonClasses) {
        button.className += ` ${actionButtonClasses}`;
      }

      button.addEventListener("click", (e) => {
        e.stopPropagation(); // Prevent triggering swipe events
        const action = options.actions[index];
        if (action.onClick) {
          action.onClick();
        }
        // Auto-dismiss toast after action unless specified otherwise
        if (action.dismissOnClick !== false) {
          dismissToast(toastItem);
        }
      });
    });
  }

  return toastItem;
}

// Update front status for all toasts in the wrapper (array-based, like Sonner)
function updateFrontStatus(toastsWrapper) {
  if (!toastsWrapper) return;

  const toasts = getToastArray(toastsWrapper);

  // Filter out toasts that are being removed
  const activeToasts = toasts.filter(
    (toast) =>
      toast.element.dataset.removed !== "true" &&
      toast.element.dataset.swipeOut !== "true",
  );

  // Mark the last toast (most recent, highest index) as front
  activeToasts.forEach((toast, index) => {
    toast.element.dataset.front = index === activeToasts.length - 1 ? "true" : "false";
  });

  // Update visible state
  updateVisibleToasts(toastsWrapper);

  // Update inline CSS variables (index, z-index)
  updateToastStyles(toastsWrapper);
}

// Update visible toasts based on VISIBLE_TOASTS_AMOUNT
function updateVisibleToasts(toastsWrapper) {
  const toasts = getToastArray(toastsWrapper);
  const activeToasts = getActiveToasts(toasts);
  const maxToasts = getCSSValue(toastsWrapper, TOAST_PROPS.MAX_TOASTS) || 5;

  // Set data-visible and data-hidden based on position from end (most recent toasts)
  activeToasts.forEach((toast, index) => {
    const fromEnd = activeToasts.length - 1 - index;
    toast.element.dataset.visible = fromEnd < VISIBLE_TOASTS_AMOUNT ? "true" : "false";
    // Hide toasts beyond max limit (use JS instead of CSS nth-last-child to handle removed toasts)
    toast.element.dataset.hidden = fromEnd >= maxToasts ? "true" : "false";
  });
}

// Update inline CSS variables for all toasts (index, z-index, etc.)
function updateToastStyles(toastsWrapper) {
  const toasts = getToastArray(toastsWrapper);
  const activeToasts = getActiveToasts(toasts);

  // Update styles for each toast based on position
  activeToasts.forEach((toast, arrayIndex) => {
    // In Sonner: index 0 = front (most recent), higher index = older
    // Our array: last item = most recent, first item = oldest
    // So we need to reverse: index = length - 1 - arrayIndex
    const index = activeToasts.length - 1 - arrayIndex;
    const zIndex = activeToasts.length - index;

    // Set inline CSS variables
    toast.element.style.setProperty("--index", index.toString());
    toast.element.style.setProperty("--toasts-before", index.toString());
    toast.element.style.setProperty("--z-index", zIndex.toString());
  });
}

// Setup expanded state tracking for hover/focus
function setupExpandedState(toastsWrapper) {
  let isExpanded = false;

  const setExpanded = (expanded) => {
    if (isExpanded === expanded) return;
    isExpanded = expanded;

    // Update wrapper attribute
    toastsWrapper.dataset.expanded = expanded ? "true" : "false";

    // Update all toasts in this wrapper with transforms
    const toasts = getToastArray(toastsWrapper);
    const activeToasts = getActiveToasts(toasts);
    const gap = getCSSValue(toastsWrapper, "--gap") || 15;

    // Calculate cumulative offset for each toast (based on heights)
    let cumulativeOffset = 0;
    const offsets = [];

    // Process from newest (last) to oldest (first) - newest has offset 0
    for (let i = activeToasts.length - 1; i >= 0; i--) {
      offsets[i] = cumulativeOffset;
      const height = activeToasts[i].element.getBoundingClientRect().height;
      cumulativeOffset += height + gap;
    }

    activeToasts.forEach((toast, arrayIndex) => {
      const el = toast.element;
      el.dataset.expanded = expanded ? "true" : "false";

      // Skip transform updates for toasts still in entry animation
      // This allows CSS to handle the entry animation without JS override
      if (el.dataset.mounted === "false" || el.dataset.entering === "true") {
        return;
      }

      // Get lift direction from toast's data attribute
      const lift = el.style.getPropertyValue("--lift") || "-1";
      const liftValue = Number.parseFloat(lift);

      // Sonner index: 0 = front (newest), higher = older
      const index = activeToasts.length - 1 - arrayIndex;
      const offset = offsets[arrayIndex];

      if (expanded) {
        // Expanded: spread toasts using offset and lift direction
        const translateY = liftValue * offset;
        el.style.transform = `translateY(${translateY}px)`;
        el.style.height = "auto";
      } else {
        // Collapsed: stack toasts with scale effect
        if (index === 0) {
          // Front toast - no transform
          el.style.transform = "translateY(0)";
          el.style.height = "auto";
        } else {
          // Background toasts - stack with gap and scale
          const stackGap = getCSSValue(toastsWrapper, "--stack-spacing") || 10;
          const scaleFactor = getCSSValue(toastsWrapper, "--scale-factor") || 0.05;
          const translateY = liftValue * stackGap * index;
          const scale = 1 - scaleFactor * index;
          el.style.transform = `translateY(${translateY}px) scale(${scale})`;
        }
      }
    });
  };

  toastsWrapper.addEventListener("mouseenter", () => setExpanded(true));
  toastsWrapper.addEventListener("mousemove", () => setExpanded(true));
  toastsWrapper.addEventListener("mouseleave", () => setExpanded(false));
  toastsWrapper.addEventListener("focus", () => setExpanded(true), true);
  toastsWrapper.addEventListener("blur", () => setExpanded(false), true);
}

// Dismiss toast utility
function dismissToast(toast) {
  if (toast.dataset.removed === "true" || toast.dataset.swipeOut === "true") {
    return; // Already dismissing
  }

  // Cleanup timer if it exists
  if (toast._cleanupTimer) {
    toast._cleanupTimer();
  }

  // Mark as removed (not swipe out)
  toast.dataset.removed = "true";
  toast.dataset.swipeOut = "false";

  const toastsWrapper = toast.parentNode;

  // Update front status for all toasts
  updateFrontStatus(toastsWrapper);

  // Wait for transition to complete
  const exitDuration = getCSSValue(toastsWrapper, "--exit-duration");

  setTimeout(() => {
    // Remove from state array
    removeToastFromState(toastsWrapper, toast);

    // Remove from DOM
    if (toast.parentNode) {
      toast.parentNode.removeChild(toast);
    }
  }, exitDuration || 200);
}

// Setup swipe-to-dismiss functionality
function setupSwipeToDismiss(toast) {
  let pointerStart = null;
  let swipeDirection = null;
  const position = "BottomRight"; // Get from data attribute if needed
  const swipeDirections = getSwipeDirections(position);
  let dragStartTime = null;

  const handlePointerDown = (event) => {
    // Ignore right-click
    if (event.button === 2) return;

    // Don't allow swiping during dismissal
    if (toast.dataset.removed === "true" || toast.dataset.swipeOut === "true") {
      return;
    }

    // Set pointer capture for smooth dragging outside element
    event.target.setPointerCapture(event.pointerId);

    dragStartTime = new Date();
    pointerStart = { x: event.clientX, y: event.clientY };
    toast.dataset.swiping = "true";
  };

  const handlePointerMove = (event) => {
    if (!pointerStart) return;

    // Don't track if user is selecting text
    const isHighlighted = window.getSelection()?.toString().length > 0;
    if (isHighlighted) return;

    const xDelta = event.clientX - pointerStart.x;
    const yDelta = event.clientY - pointerStart.y;

    // Determine swipe direction if not locked
    if (!swipeDirection && (Math.abs(xDelta) > 1 || Math.abs(yDelta) > 1)) {
      swipeDirection = Math.abs(xDelta) > Math.abs(yDelta) ? "x" : "y";
    }

    const swipeAmount = { x: 0, y: 0 };

    // Apply swipe in the locked direction
    if (swipeDirection === "y") {
      if (swipeDirections.includes("up") || swipeDirections.includes("down")) {
        if (
          (swipeDirections.includes("up") && yDelta < 0) ||
          (swipeDirections.includes("down") && yDelta > 0)
        ) {
          swipeAmount.y = yDelta;
        } else {
          // Dampened movement for wrong direction
          const dampenedDelta = yDelta * getDampening(yDelta);
          swipeAmount.y =
            Math.abs(dampenedDelta) < Math.abs(yDelta) ? dampenedDelta : yDelta;
        }
      }
    } else if (swipeDirection === "x") {
      if (swipeDirections.includes("left") || swipeDirections.includes("right")) {
        if (
          (swipeDirections.includes("left") && xDelta < 0) ||
          (swipeDirections.includes("right") && xDelta > 0)
        ) {
          swipeAmount.x = xDelta;
        } else {
          // Dampened movement for wrong direction
          const dampenedDelta = xDelta * getDampening(xDelta);
          swipeAmount.x =
            Math.abs(dampenedDelta) < Math.abs(xDelta) ? dampenedDelta : xDelta;
        }
      }
    }

    // Apply CSS custom properties for transform
    toast.style.setProperty("--swipe-amount-x", `${swipeAmount.x}px`);
    toast.style.setProperty("--swipe-amount-y", `${swipeAmount.y}px`);
  };

  const handlePointerUp = () => {
    if (!pointerStart) return;

    const swipeAmountX = Number.parseFloat(
      toast.style.getPropertyValue("--swipe-amount-x").replace("px", "") || 0,
    );
    const swipeAmountY = Number.parseFloat(
      toast.style.getPropertyValue("--swipe-amount-y").replace("px", "") || 0,
    );

    const timeTaken = Date.now() - dragStartTime.getTime();
    const swipeAmount = swipeDirection === "x" ? swipeAmountX : swipeAmountY;
    const velocity = Math.abs(swipeAmount) / timeTaken;

    // Check if swipe threshold met (distance or velocity)
    if (Math.abs(swipeAmount) >= SWIPE_THRESHOLD || velocity > SWIPE_VELOCITY_THRESHOLD) {
      // Determine swipe-out direction
      let swipeOutDirection;
      if (swipeDirection === "x") {
        swipeOutDirection = swipeAmountX > 0 ? "right" : "left";
      } else {
        swipeOutDirection = swipeAmountY > 0 ? "down" : "up";
      }

      // Cleanup timer if it exists
      if (toast._cleanupTimer) {
        toast._cleanupTimer();
      }

      // Set swipe-out attributes
      toast.dataset.swipeOut = "true";
      toast.dataset.removed = "true";
      // Convert to PascalCase (capitalize first letter)
      toast.dataset.swipeDirection =
        swipeOutDirection.charAt(0).toUpperCase() + swipeOutDirection.slice(1);
      toast.dataset.swiping = "false";

      const toastsWrapper = toast.parentNode;

      // Update front status for all toasts
      updateFrontStatus(toastsWrapper);

      // Wait for animation to complete before removing
      const exitDuration = getCSSValue(toastsWrapper, "--exit-duration");

      setTimeout(() => {
        // Remove from state array
        removeToastFromState(toastsWrapper, toast);

        // Remove from DOM
        if (toast.parentNode) {
          toast.parentNode.removeChild(toast);
        }
      }, exitDuration || 200);
    } else {
      // Reset swipe - spring back to original position
      toast.style.setProperty("--swipe-amount-x", "0px");
      toast.style.setProperty("--swipe-amount-y", "0px");
      toast.dataset.swiping = "false";
    }

    // Reset swipe state
    pointerStart = null;
    swipeDirection = null;
    dragStartTime = null;
  };

  toast.addEventListener("pointerdown", handlePointerDown);
  toast.addEventListener("pointermove", handlePointerMove);
  toast.addEventListener("pointerup", handlePointerUp);
  toast.addEventListener("pointercancel", handlePointerUp);
}

// Mount toast - trigger enter animation via data-mounted
function mountToast(toast) {
  // data-entering="true" is already set in createToastElement to prevent
  // stacking transforms from overriding entry animation

  // Set mounted to true using requestAnimationFrame to trigger CSS transition
  requestAnimationFrame(() => {
    requestAnimationFrame(() => {
      toast.dataset.mounted = "true";

      // Update front status after entrance duration
      const toastsWrapper = toast.parentNode;
      if (toastsWrapper) {
        const enterDuration = getCSSValue(toastsWrapper, "--enter-duration");

        setTimeout(() => {
          // Remove entering flag after animation completes
          toast.dataset.entering = "false";

          if (toast.parentNode) {
            updateFrontStatus(toast.parentNode);
          }
        }, enterDuration || 300);
      }
    });
  });
}

// Setup pause on hover timer functionality
function setupPauseOnHover(
  toast,
  toastsWrapper,
  startMode = TIMER_START.ON_ANIMATION_END,
  customDuration = null,
) {
  const duration = customDuration || getCSSValue(toastsWrapper, "--dismiss-delay");

  // Skip timer setup for loading toasts
  if (toast.dataset.variant === "Loading") return;

  const progressBar = toast.querySelector("[data-duration-progress]");
  if (!progressBar) return;

  // Clean up existing timer if present
  if (toast._cleanupTimer) {
    toast._cleanupTimer();
  }

  let startTime = null;
  let remainingTime = duration;
  let timerId = null;
  let isPaused = false;

  const startTimer = () => {
    if (isPaused) return;

    startTime = Date.now();

    // Animate progress bar - ensure we start from scaleX(1)
    // First, reset to initial state without transition
    progressBar.style.transition = "none";
    progressBar.style.transform = "scaleX(1)";

    // Force reflow to ensure the initial state is applied
    progressBar.offsetHeight;

    // Now apply the transition and animate to scaleX(0)
    progressBar.style.transition = `transform ${remainingTime}ms linear`;
    progressBar.style.transform = "scaleX(0)";

    // Set timer to auto-dismiss
    timerId = setTimeout(() => {
      dismissToast(toast);
    }, remainingTime);
  };

  const pauseTimer = () => {
    if (isPaused) return;
    isPaused = true;

    clearTimeout(timerId);

    // Calculate remaining time
    const elapsed = Date.now() - startTime;
    remainingTime = Math.max(0, remainingTime - elapsed);

    // Pause progress bar animation by capturing current scale
    const computedStyle = window.getComputedStyle(progressBar);
    const matrix = new DOMMatrix(computedStyle.transform);
    const currentScale = matrix.a; // scaleX value

    progressBar.style.transition = "none";
    progressBar.style.transform = `scaleX(${currentScale})`;
  };

  const resumeTimer = () => {
    if (!isPaused) return;
    isPaused = false;

    // Force a reflow to ensure the paused state is applied
    progressBar.offsetHeight;

    startTimer();
  };

  // Listen for hover events on the toast list
  toastsWrapper.addEventListener("mouseenter", pauseTimer);
  toastsWrapper.addEventListener("mouseleave", resumeTimer);

  // Start timer based on mode
  if (startMode === TIMER_START.IMMEDIATELY || toast.dataset.mounted === "true") {
    startTimer();
  } else {
    // Start the timer after mount completes (entrance duration)
    const enterDuration = getCSSValue(toastsWrapper, "--enter-duration");

    setTimeout(() => {
      if (toast.dataset.removed !== "true") {
        startTimer();
      }
    }, enterDuration || 300);
  }

  // Store cleanup function on toast element
  toast._cleanupTimer = () => {
    clearTimeout(timerId);
    toastsWrapper.removeEventListener("mouseenter", pauseTimer);
    toastsWrapper.removeEventListener("mouseleave", resumeTimer);
  };
}

// Simple toast creation - CSS handles everything else
function createNewToast(toastsWrapper, variant = "Default", options = {}) {
  // Check if single-mode is enabled (from options or container attribute)
  const isSingleMode =
    options.singleMode !== undefined
      ? options.singleMode
      : toastsWrapper.getAttribute("data-single-mode") === "true";

  // Get max toasts from CSS custom property
  const maxToasts = getCSSValue(toastsWrapper, TOAST_PROPS.MAX_TOASTS);

  // Get toasts from state array instead of DOM
  const toasts = getToastArray(toastsWrapper);

  // Count only active toasts (not removed)
  const activeToasts = getActiveToasts(toasts);

  // If single-mode, dismiss all existing toasts
  if (isSingleMode && activeToasts.length > 0) {
    activeToasts.forEach((toast) => {
      dismissToast(toast.element);
    });
  }
  // Otherwise, remove oldest if at limit
  else if (activeToasts.length >= maxToasts) {
    const oldestToast = activeToasts[0];
    if (oldestToast) dismissToast(oldestToast.element);
  }

  // Create and add new toast
  const newToast = createToastElement(variant, options);

  // Set position attributes from toaster (critical for expansion direction)
  const toasterPosition = toastsWrapper.getAttribute("data-position");
  const { y, x } = parsePosition(toasterPosition);
  newToast.dataset.yPosition = y;
  newToast.dataset.xPosition = x;

  // Add positioning class based on y-position
  newToast.classList.add(y === "Top" ? "top-0" : "bottom-0");

  // Set --lift CSS variable for expansion direction
  // Top positions: --lift: 1 (expand downward into page)
  // Bottom positions: --lift: -1 (expand upward into page)
  newToast.style.setProperty("--lift", y === "Top" ? "1" : "-1");

  // Prepend toast (newest first in DOM, like original Sonner)
  toastsWrapper.prepend(newToast);

  // Add to state array
  addToastToState(toastsWrapper, newToast);

  // Mount toast - triggers enter animation
  mountToast(newToast);

  // Setup swipe-to-dismiss
  setupSwipeToDismiss(newToast);

  // Setup pause on hover with timer
  setupPauseOnHover(newToast, toastsWrapper);

  // Update front status after adding
  updateFrontStatus(toastsWrapper);

  // Return toast element for promise toasts
  return newToast;
}

// Update toast content and variant (for promise toasts)
function updateToast(toast, newVariant, newContent, customIcon = null, options = {}) {
  const toastsWrapper = toast.parentNode;

  // Update variant and classes
  toast.dataset.variant = newVariant;

  // Update Tailwind classes for new variant
  const variantClasses = VARIANT_CLASSES[newVariant] || VARIANT_CLASSES.Default;
  toast.className =
    toast.className.replace(/bg-\S+|text-\S+|border-\S+/g, "").trim() +
    ` ${variantClasses}`;

  // Update icon (use custom icon if provided, otherwise use default for variant)
  const iconContainer = toast.querySelector("[data-icon]");
  const newIcon = customIcon || TOAST_ICONS[newVariant.toLowerCase()];
  if (iconContainer && newIcon) {
    iconContainer.innerHTML = newIcon;
  }

  // Update title
  const titleElement = toast.querySelector("h3");
  if (titleElement && newContent.title) {
    titleElement.textContent = newContent.title;
  }

  // Update description
  const descriptionElement = toast.querySelector("p");
  if (newContent.description) {
    if (descriptionElement) {
      descriptionElement.textContent = newContent.description;
    } else {
      // Add description if it didn't exist
      const titleContainer = toast.querySelector(".flex-1");
      const descriptionHtml = `<p class="text-sm leading-[1.5] opacity-90 mt-1">${newContent.description}</p>`;
      titleContainer.insertAdjacentHTML("beforeend", descriptionHtml);
    }
  } else if (descriptionElement) {
    // Remove description if new content doesn't have one
    descriptionElement.remove();
  }

  // Add close button if it wasn't there (loading toasts don't have close buttons)
  if (newVariant !== "Loading" && !toast.querySelector("[data-close-button]")) {
    const titleContainer = toast.querySelector(".flex-1 .flex");
    const closeButtonHtml = `<button data-close-button aria-label="Close toast" type="button" class="w-5 h-5 flex items-center justify-center border-none bg-transparent text-muted-foreground cursor-pointer transition-colors duration-150 shrink-0 p-0 ml-auto hover:text-foreground">${CLOSE_ICON}</button>`;
    titleContainer.insertAdjacentHTML("beforeend", closeButtonHtml);

    const closeButton = toast.querySelector("[data-close-button]");
    closeButton.addEventListener("click", (e) => {
      e.stopPropagation();
      dismissToast(toast);
    });
  }

  // Handle duration track for promise toasts
  const durationTrack = toast.querySelector("[data-duration-track]");
  if (options.hideProgressBar && durationTrack) {
    durationTrack.style.display = "none";
  }

  // Setup timer for non-loading, non-promise toasts
  if (newVariant !== "Loading" && toastsWrapper && !options.hideProgressBar) {
    setupPauseOnHover(toast, toastsWrapper, TIMER_START.IMMEDIATELY);
  }
}

// Promise toast - shows loading state and updates based on promise result
function toastPromise(toastsWrapper, promise, messages) {
  // Create loading toast
  const toast = createNewToast(toastsWrapper, "Loading", {
    content: {
      title: messages.loading,
    },
  });

  // Handle promise resolution
  promise
    .then((data) => {
      // Update to default state with filled checkmark icon
      const successMessage =
        typeof messages.success === "function"
          ? messages.success(data)
          : messages.success;

      updateToast(
        toast,
        "Default",
        {
          title: successMessage,
        },
        CHECK_FILLED_ICON,
        {
          hideProgressBar: true,
        },
      );
    })
    .catch((err) => {
      // Update to error state with filled error icon
      const errorMessage =
        typeof messages.error === "function" ? messages.error(err) : messages.error;

      updateToast(
        toast,
        "Error",
        {
          title: errorMessage,
        },
        ERROR_FILLED_ICON,
        {
          hideProgressBar: true,
        },
      );
    });

  return toast;
}
