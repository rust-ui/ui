// Bundled resizable scripts for performance optimization
(() => {
  // ========== RESIZABLE BASE ==========
  const initResize = (e) => {
    e.preventDefault();

    const handle = e.target;
    const rightPanel = handle.nextElementSibling;
    const container = handle.parentElement;
    const startPos = e.clientX;
    const startSize = Number.parseInt(getComputedStyle(rightPanel).width, 10);
    const containerWidth = container.getBoundingClientRect().width;

    document.documentElement.style.cursor = "col-resize";
    document.body.classList.add("pointer-events-none", "select-none");

    const doResize = (e) => {
      const delta = startPos - e.clientX;
      const newSize = Math.max(0, Math.min(containerWidth * 0.8, startSize + delta));
      rightPanel.style.width = `${newSize}px`;
    };

    const stopResize = () => {
      document.documentElement.style.cursor = "";
      document.body.classList.remove("pointer-events-none", "select-none");
      document.removeEventListener("pointermove", doResize);
      document.removeEventListener("pointerup", stopResize);
    };

    document.addEventListener("pointermove", doResize);
    document.addEventListener("pointerup", stopResize);
  };

  function initializeResizable() {
    const handles = document.querySelectorAll('[data-name="ResizableHandle"]');

    handles.forEach((handle) => {
      if (handle.hasAttribute("data-resizable-initialized")) {
        return;
      }
      handle.setAttribute("data-resizable-initialized", "true");
      handle.addEventListener("pointerdown", initResize);
    });
  }

  // ========== RESIZABLE LEPTOS ==========
  const resizables = new Map();

  const emitError = (message, instanceId = null) => {
    document.dispatchEvent(
      new CustomEvent("resizable:error", {
        detail: { error: message, instanceId },
      }),
    );
  };

  const emitSuccess = (action, instanceId, data = {}) => {
    document.dispatchEvent(
      new CustomEvent("resizable:success", {
        detail: { action, instanceId, ...data },
      }),
    );
  };

  const setSizeByScreenType = (instanceId, screenType) => {
    const resizable = resizables.get(instanceId);

    if (!resizable) {
      emitError(
        `Resizable instance ${instanceId} not found. Available instances: [${Array.from(resizables.keys()).join(", ")}]`,
        instanceId,
      );
      return;
    }

    const { background, parent } = resizable;
    const containerWidth = parent.getBoundingClientRect().width;

    let targetPreviewWidth;
    switch (screenType) {
      case "Desktop":
        targetPreviewWidth = containerWidth;
        break;
      case "Tablet":
        targetPreviewWidth = 768;
        break;
      case "Phone":
        targetPreviewWidth = 375;
        break;
      default:
        targetPreviewWidth = containerWidth;
        emitError(`Unknown screen type: ${screenType}`, instanceId);
    }

    const backgroundWidth = Math.max(0, containerWidth - targetPreviewWidth);

    background.style.transition = "width 0.3s ease-out";
    background.style.width = `${backgroundWidth}px`;

    emitSuccess("resizable-resized", instanceId, {
      screenType,
      targetPreviewWidth,
      backgroundWidth,
    });
  };

  document.addEventListener("resizable:resize_by_screen__interop", (event) => {
    const { instanceId, screenType } = event.detail;
    setSizeByScreenType(instanceId, screenType);
  });

  function initializeResizableEvents() {
    const containers = document.querySelectorAll('[data-name="ResizableContainer"]');

    containers.forEach((container) => {
      const parent = container.parentElement;
      if (!parent) return;

      const handle = parent.querySelector('[data-name="ResizableHandle"]');
      const background = parent.querySelector('[data-name="ResizableBackground"]');

      if (!handle || !background) return;

      if (parent.hasAttribute("data-resizable-events-initialized")) {
        return;
      }
      parent.setAttribute("data-resizable-events-initialized", "true");

      let instanceId = null;
      let element = parent;
      while (element && !instanceId) {
        if (element.id) {
          instanceId = element.id;
          break;
        }
        element = element.parentElement;
      }

      if (instanceId) {
        resizables.set(instanceId, {
          container,
          background,
          handle,
          parent,
        });

        emitSuccess("resizable-initialized", instanceId, {
          containerId: container.id || "unnamed",
        });
      }
    });
  }

  // ========== UNIFIED INITIALIZATION ==========
  function initializeAll() {
    initializeResizable();
    initializeResizableEvents();
  }

  initializeAll();

  const observer = new MutationObserver((mutations) => {
    mutations.forEach((mutation) => {
      if (mutation.type === "childList" && mutation.addedNodes.length > 0) {
        initializeAll();
      }
    });
  });

  // Wait for DOM to be ready before observing
  if (document.body) {
    observer.observe(document.body, {
      childList: true,
      subtree: true,
    });
  } else {
    document.addEventListener("DOMContentLoaded", () => {
      observer.observe(document.body, {
        childList: true,
        subtree: true,
      });
    });
  }
})();
