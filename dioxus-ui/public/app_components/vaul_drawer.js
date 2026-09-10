// Constants
const VELOCITY_THRESHOLD = 0.4;
const CLOSE_THRESHOLD = 0.25;
const TRANSITION_DURATION = 500; // ms
const WINDOW_TOP_OFFSET = 26; // px
const BORDER_RADIUS = 8; // px
const SCROLL_LOCK_TIMEOUT = 500; // ms - prevents dragging after scrolling

// Helper to calculate scale
function getScale() {
  return (window.innerWidth - WINDOW_TOP_OFFSET) / window.innerWidth;
}

// Damping function - adds resistance when dragging beyond limits
function dampenValue(v) {
  return 8 * (Math.log(v + 1) - 2);
}

// Initialize a single drawer instance
function initDrawerInstance(drawer, index) {
  // Find the trigger and close button for this drawer
  // We look for the trigger that comes before this drawer in the DOM
  const allTriggers = Array.from(
    document.querySelectorAll('[data-name="DrawerTrigger"]'),
  );
  const trigger = allTriggers[index];

  // Find shared overlay
  const overlay = document.querySelector('[data-name="DrawerOverlay"]');
  const closeBtn = drawer.querySelector('[data-name="DrawerClose"]');
  const wrapper = document.querySelector("[data-vaul-drawer-wrapper]");

  // Detect drawer direction and properties
  const position = drawer.getAttribute("data-vaul-drawer-position") || "Bottom";
  const isHorizontal = position === "Left" || position === "Right";
  const isFloating = drawer.getAttribute("data-vaul-variant") === "Floating";
  const isDismissible = drawer.getAttribute("data-vaul-dismissible") !== "false";
  const lockBodyScroll = overlay?.getAttribute("data-lock-body-scroll") !== "false";

  // State for this drawer instance
  let isOpen = false;
  let isDragging = false;
  let startPos = 0;
  let currentPos = 0;
  let drawerSize = 0;
  let dragStartTime = 0;
  let previousActiveElement = null;
  let openTime = null;
  let lastTimeDragPrevented = null;
  let isAllowedToDrag = false;

  // Get all focusable elements in the drawer for tab trapping
  function getFocusableElements() {
    const focusableSelectors = [
      "a[href]",
      "button:not([disabled])",
      "textarea:not([disabled])",
      "input:not([disabled])",
      "select:not([disabled])",
      '[tabindex]:not([tabindex="-1"])',
    ].join(", ");

    return Array.from(drawer.querySelectorAll(focusableSelectors));
  }

  // Check if element should allow dragging (Vaul's scroll detection logic)
  function shouldDrag(target) {
    let element = target;
    const currentDate = Date.now();

    if (openTime && currentDate - openTime < 500) {
      return false;
    }

    if (
      lastTimeDragPrevented &&
      currentDate - lastTimeDragPrevented < SCROLL_LOCK_TIMEOUT
    ) {
      lastTimeDragPrevented = currentDate;
      return false;
    }

    while (element && element !== drawer) {
      if (element.scrollHeight > element.clientHeight) {
        if (element.scrollTop !== 0) {
          lastTimeDragPrevented = currentDate;
          return false;
        }

        if (element.getAttribute("role") === "dialog") {
          return true;
        }
      }

      element = element.parentElement;
    }

    return true;
  }

  // Fix drawer position to viewport bottom
  // * This allows us to have the data-vaul-drawer-wrapper="" in app.rs and
  // * thus to make the depth effect to the entire app.
  function fixDrawerPosition() {
    // Return early if there's no scroll on the page (no need to fix position)
    const hasScroll = document.documentElement.scrollHeight > window.innerHeight;
    if (!hasScroll) {
      return;
    }

    const viewportHeight = window.innerHeight;
    const drawerRect = drawer.getBoundingClientRect();

    // Calculate where the drawer should be (at viewport bottom)
    const targetBottom = viewportHeight;
    const currentBottom = drawerRect.bottom;

    // Calculate offset needed
    const offset = targetBottom - currentBottom;

    // Apply position fix using top CSS property
    if (offset !== 0) {
      const currentTop = drawerRect.top;
      drawer.style.top = `${currentTop + offset}px`;
    }
  }

  // Open drawer
  function openDrawer() {
    // If drawer is already open, don't re-run open logic
    // Content updates are handled by Leptos reactivity
    if (isOpen) {
      return;
    }

    isOpen = true;
    openTime = Date.now();

    previousActiveElement = document.activeElement;

    if (isFloating) {
      overlay.style.opacity = "1";
    }

    overlay.classList.remove("hidden");
    drawer.classList.remove("hidden");

    // Conditionally lock body scroll
    if (lockBodyScroll) {
      // Calculate scrollbar width for compensation before body overflow changes
      const body = document.body;
      const scrollbarWidth = window.innerWidth - body.clientWidth;

      document.body.setAttribute("data-state", "open");

      // Add padding-right to body to compensate for scrollbar removal
      if (scrollbarWidth > 0) {
        body.style.paddingRight = `${scrollbarWidth}px`;
      }
    }

    // Fix position to viewport bottom to have data-vaul-drawer-wrapper="" accessible from app.rs
    // Only fix position when body scroll is locked
    if (lockBodyScroll) {
      fixDrawerPosition();
    }

    if (wrapper) {
      const scale = getScale();
      wrapper.style.transformOrigin = "top";
      wrapper.style.transitionProperty = "transform, border-radius";
      wrapper.style.transitionDuration = "0.5s";
      wrapper.style.transitionTimingFunction = "cubic-bezier(0.32, 0.72, 0, 1)";
      wrapper.style.borderRadius = `${BORDER_RADIUS}px`;
      wrapper.style.overflow = "hidden";
      wrapper.style.transform = `scale(${scale}) translate3d(0, calc(env(safe-area-inset-top) + 14px), 0)`;

      document.body.style.background = "black";
    }

    requestAnimationFrame(() => {
      if (isFloating) {
        overlay.setAttribute("data-vaul-animate", "false");
      }

      overlay.setAttribute("data-state", "open");
      drawer.setAttribute("data-state", "open");

      setTimeout(() => {
        const focusableElements = getFocusableElements();
        if (focusableElements.length > 0) {
          const isOnlyCloseButton =
            focusableElements.length === 1 &&
            focusableElements[0].getAttribute("data-name") === "DrawerClose";

          if (isOnlyCloseButton) {
            drawer.focus();
          } else {
            focusableElements[0].focus();
          }
        }
      }, 100);

      setTimeout(() => {}, TRANSITION_DURATION);
    });
  }

  // Close drawer
  function closeDrawer() {
    isOpen = false;

    drawerSize = isHorizontal
      ? drawer.getBoundingClientRect().width
      : drawer.getBoundingClientRect().height;

    drawer.setAttribute("data-vaul-animate", "false");
    overlay.setAttribute("data-vaul-animate", "false");

    drawer.style.transition = "transform 0.5s cubic-bezier(0.32, 0.72, 0, 1)";

    let closeTransform;
    if (isHorizontal) {
      const closeDistance = isFloating ? drawerSize + 8 : drawerSize;
      const xValue = position === "Right" ? closeDistance : -closeDistance;
      closeTransform = `translate3d(${xValue}px, 0, 0)`;
    } else {
      const yValue = drawerSize;
      closeTransform = `translate3d(0, ${yValue}px, 0)`;
    }
    drawer.style.transform = closeTransform;

    overlay.style.transition = "opacity 0.5s cubic-bezier(0.32, 0.72, 0, 1)";
    overlay.style.opacity = "0";

    if (wrapper) {
      wrapper.style.transition =
        "transform 0.5s cubic-bezier(0.32, 0.72, 0, 1), border-radius 0.5s cubic-bezier(0.32, 0.72, 0, 1)";
      wrapper.style.transform = "scale(1) translate3d(0, 0, 0)";
      wrapper.style.borderRadius = "0px";
    }

    overlay.setAttribute("data-state", "closed");
    drawer.setAttribute("data-state", "closed");

    // Conditionally unlock body scroll
    if (lockBodyScroll) {
      document.body.removeAttribute("data-state");
      document.body.style.paddingRight = ""; // Reset scrollbar compensation immediately
    }

    setTimeout(() => {
      overlay.classList.add("hidden");
      drawer.classList.add("hidden");

      drawer.style.transform = "";
      drawer.style.transition = "";
      drawer.style.top = ""; // Reset top position
      overlay.style.opacity = "";
      overlay.style.transition = "";
      drawer.setAttribute("data-vaul-animate", "true");
      overlay.setAttribute("data-vaul-animate", "true");

      if (wrapper) {
        wrapper.style.overflow = "";
        document.body.style.background = "";
      }

      if (previousActiveElement && typeof previousActiveElement.focus === "function") {
        previousActiveElement.focus();
        previousActiveElement = null;
      }
    }, TRANSITION_DURATION);
  }

  // Handle pointer down
  function onPointerDown(event) {
    if (!isOpen) return;

    if (!drawer.contains(event.target)) return;

    // Don't interfere with close button clicks
    const isCloseButton = event.target.closest('[data-name="DrawerClose"]');
    if (isCloseButton) return;

    isDragging = true;
    isAllowedToDrag = false;
    startPos = isHorizontal ? event.pageX : event.pageY;
    currentPos = startPos;
    dragStartTime = Date.now();
    drawerSize = isHorizontal
      ? drawer.getBoundingClientRect().width
      : drawer.getBoundingClientRect().height;

    drawer.style.transition = "none";

    drawer.setPointerCapture(event.pointerId);
  }

  // Handle pointer move
  function onPointerMove(event) {
    if (!isDragging) return;

    currentPos = isHorizontal ? event.pageX : event.pageY;
    const delta = currentPos - startPos;

    let isDraggingInClosingDirection = false;
    if (position === "Bottom" || position === "Right") {
      isDraggingInClosingDirection = delta > 0;
    } else {
      isDraggingInClosingDirection = delta < 0;
    }

    if (!isAllowedToDrag && !shouldDrag(event.target)) {
      return;
    }

    isAllowedToDrag = true;

    if (isDraggingInClosingDirection) {
      const absDelta = Math.abs(delta);
      const transform = isHorizontal
        ? `translate3d(${delta}px, 0, 0)`
        : `translate3d(0, ${delta}px, 0)`;
      drawer.style.transform = transform;

      if (wrapper) {
        const percentageDragged = Math.min(absDelta / drawerSize, 1);

        const scaleValue = Math.min(getScale() + percentageDragged * (1 - getScale()), 1);

        const borderRadiusValue = Math.max(
          0,
          BORDER_RADIUS - percentageDragged * BORDER_RADIUS,
        );

        const translateValue = Math.max(0, 14 - percentageDragged * 14);

        wrapper.style.transition = "none";
        wrapper.style.borderRadius = `${borderRadiusValue}px`;
        wrapper.style.transform = `scale(${scaleValue}) translate3d(0, ${translateValue}px, 0)`;
      }

      if (overlay) {
        const percentageDragged = Math.min(absDelta / drawerSize, 1);
        const opacityValue = Math.max(0, 1 - percentageDragged);
        overlay.style.transition = "none";
        overlay.style.opacity = String(opacityValue);
      }
    } else {
      const absDelta = Math.abs(delta);
      const dampedDelta = dampenValue(absDelta);
      const signedDampedDelta = delta > 0 ? dampedDelta : -dampedDelta;
      const transform = isHorizontal
        ? `translate3d(${signedDampedDelta}px, 0, 0)`
        : `translate3d(0, ${-dampedDelta}px, 0)`;
      drawer.style.transform = transform;
    }
  }

  // Handle pointer up
  function onPointerUp(event) {
    if (!isDragging) return;

    isDragging = false;
    const delta = currentPos - startPos;
    const dragEndTime = Date.now();
    const timeTaken = dragEndTime - dragStartTime;
    const velocity = Math.abs(delta) / timeTaken;

    drawer.style.transition = "transform 0.5s cubic-bezier(0.32, 0.72, 0, 1)";

    if (wrapper) {
      wrapper.style.transition =
        "transform 0.5s cubic-bezier(0.32, 0.72, 0, 1), border-radius 0.5s cubic-bezier(0.32, 0.72, 0, 1)";
    }

    if (overlay) {
      overlay.style.transition = "opacity 0.5s cubic-bezier(0.32, 0.72, 0, 1)";
    }

    let shouldClose = false;
    if (position === "Bottom" || position === "Right") {
      shouldClose =
        (velocity > VELOCITY_THRESHOLD || delta / drawerSize >= CLOSE_THRESHOLD) &&
        delta > 0;
    } else {
      shouldClose =
        (velocity > VELOCITY_THRESHOLD ||
          Math.abs(delta) / drawerSize >= CLOSE_THRESHOLD) &&
        delta < 0;
    }

    if (shouldClose) {
      closeDrawer();
    } else {
      drawer.style.transform = "translate3d(0, 0, 0)";

      if (wrapper) {
        const scale = getScale();
        wrapper.style.borderRadius = `${BORDER_RADIUS}px`;
        wrapper.style.transform = `scale(${scale}) translate3d(0, 14px, 0)`;
      }

      if (overlay) {
        overlay.style.opacity = "1";
      }
    }

    if (drawer.hasPointerCapture(event.pointerId)) {
      drawer.releasePointerCapture(event.pointerId);
    }
  }

  // Keyboard event handler
  function handleKeyDown(event) {
    if (!isOpen) return;

    if (event.key === "Escape") {
      event.preventDefault();
      closeDrawer();
      return;
    }

    if (event.key === "Tab") {
      const focusableElements = getFocusableElements();
      if (focusableElements.length === 0) return;

      const firstElement = focusableElements[0];
      const lastElement = focusableElements[focusableElements.length - 1];

      if (event.shiftKey && document.activeElement === firstElement) {
        event.preventDefault();
        lastElement.focus();
      } else if (!event.shiftKey && document.activeElement === lastElement) {
        event.preventDefault();
        firstElement.focus();
      }
    }
  }

  // Event listeners
  if (trigger) {
    trigger.addEventListener("click", openDrawer);
  }

  if (closeBtn) {
    closeBtn.addEventListener("click", closeDrawer);
  }

  // Only add dismissible features if enabled
  if (isDismissible) {
    overlay.addEventListener("click", closeDrawer);

    drawer.addEventListener("pointerdown", onPointerDown);
    drawer.addEventListener("pointermove", onPointerMove);
    drawer.addEventListener("pointerup", onPointerUp);
    drawer.addEventListener("pointercancel", onPointerUp);

    document.addEventListener("keydown", handleKeyDown);

    drawer.addEventListener("selectstart", (event) => {
      if (isDragging && isAllowedToDrag) {
        event.preventDefault();
      }
    });
  }
}

// Initialize all drawer instances
const allDrawers = document.querySelectorAll('[data-name="DrawerContent"]');
allDrawers.forEach((drawer, index) => {
  initDrawerInstance(drawer, index);
});
