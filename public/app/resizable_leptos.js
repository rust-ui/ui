(() => {
  // Map to store resizable controls by instance ID
  const resizables = new Map();

  // Helper function to emit error events
  const emitError = (message, instanceId = null) => {
    document.dispatchEvent(
      new CustomEvent("resizable:error", {
        detail: { error: message, instanceId },
      }),
    );
  };

  // Helper function to emit success events
  const emitSuccess = (action, instanceId, data = {}) => {
    document.dispatchEvent(
      new CustomEvent("resizable:success", {
        detail: { action, instanceId, ...data },
      }),
    );
  };

  // Set size based on screen size preset
  const setSizeByScreenType = (instanceId, screenType) => {
    const resizable = resizables.get(instanceId);

    if (!resizable) {
      emitError(
        `Resizable instance ${instanceId} not found. Available instances: [${Array.from(resizables.keys()).join(", ")}]`,
        instanceId,
      );
      return;
    }

    const { container, background, parent } = resizable;
    const containerWidth = parent.getBoundingClientRect().width;

    let targetPreviewWidth;
    switch (screenType) {
      case "Desktop":
        targetPreviewWidth = containerWidth; // Full width
        break;
      case "Tablet":
        targetPreviewWidth = 768; // Tablet width
        break;
      case "Phone":
        targetPreviewWidth = 375; // Phone width
        break;
      default:
        targetPreviewWidth = containerWidth;
        emitError(`Unknown screen type: ${screenType}`, instanceId);
    }

    // Calculate background width needed to achieve target preview width
    // backgroundWidth = containerWidth - targetPreviewWidth
    const backgroundWidth = Math.max(0, containerWidth - targetPreviewWidth);

    // Apply smooth transition
    background.style.transition = "width 0.3s ease-out";
    background.style.width = `${backgroundWidth}px`;

    emitSuccess("resizable-resized", instanceId, {
      screenType,
      targetPreviewWidth,
      backgroundWidth,
    });
  };

  // Listen for custom events from Rust
  document.addEventListener("resizable:resize_by_screen__interop", (event) => {
    const { instanceId, screenType } = event.detail;
    setSizeByScreenType(instanceId, screenType);
  });

  function initializeResizableEvents() {
    // Find all resizable containers - they are divs that contain ResizableHandle and ResizableBackground
    const containers = document.querySelectorAll('[data-name="ResizableContainer"]');

    containers.forEach((container) => {
      // Find the parent resizable element and siblings
      const parent = container.parentElement;
      if (!parent) return;

      const handle = parent.querySelector('[data-name="ResizableHandle"]');
      const background = parent.querySelector('[data-name="ResizableBackground"]');

      if (!handle || !background) return;

      // Skip if already initialized
      if (parent.hasAttribute("data-resizable-events-initialized")) {
        return;
      }
      parent.setAttribute("data-resizable-events-initialized", "true");

      // Try to extract instance ID from parent hierarchy
      // Look for an element with an id that might contain the instance ID
      let instanceId = null;
      let element = parent;
      while (element && !instanceId) {
        if (element.id) {
          // The instance ID might be in a parent div's id attribute
          // For BlockViewer, it's the id of the wrapper div
          instanceId = element.id;
          break;
        }
        element = element.parentElement;
      }

      if (instanceId) {
        // Store the resizable controls in the map
        resizables.set(instanceId, {
          container,
          background,
          handle,
          parent,
        });

        // Emit initialization event
        emitSuccess("resizable-initialized", instanceId, {
          containerId: container.id || "unnamed",
        });
      }
    });
  }

  // Initialize on load
  initializeResizableEvents();

  // Initialize when new elements are added
  const observer = new MutationObserver((mutations) => {
    mutations.forEach((mutation) => {
      if (mutation.type === "childList" && mutation.addedNodes.length > 0) {
        initializeResizableEvents();
      }
    });
  });

  observer.observe(document.body, {
    childList: true,
    subtree: true,
  });
})();
