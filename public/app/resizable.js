(() => {
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
      // Skip if already initialized
      if (handle.hasAttribute("data-resizable-initialized")) {
        return;
      }
      handle.setAttribute("data-resizable-initialized", "true");

      handle.addEventListener("pointerdown", initResize);
    });
  }

  // Initialize on load
  initializeResizable();

  // Initialize when new elements are added
  const observer = new MutationObserver((mutations) => {
    mutations.forEach((mutation) => {
      if (mutation.type === "childList" && mutation.addedNodes.length > 0) {
        initializeResizable();
      }
    });
  });

  observer.observe(document.body, {
    childList: true,
    subtree: true,
  });
})();
