(() => {
  let headingsArray = [];
  const TOC_CONTAINER = '[data-name="TableOfContents"]';
  const TOC_LINK = '[data-name="TocLink"]';
  const HEADINGS_SELECTOR = "h2, h3, h4, h5, h6:not(nav *, header *, footer *)";
  const SCROLL_OFFSET = 100;

  // Cache heading positions to avoid forced reflow
  let headingPositions = [];

  function cacheHeadingPositions() {
    headingPositions = headingsArray.map(heading => ({
      id: heading.id,
      top: heading.getBoundingClientRect().top + window.scrollY
    }));
  }

  function updateActiveSection() {
    const scrollPos = window.scrollY + SCROLL_OFFSET;
    let currentHeading = null;

    for (const position of headingPositions) {
      if (scrollPos >= position.top) {
        currentHeading = position;
      }
    }

    if (currentHeading?.id) {
      const newHash = `#${currentHeading.id}`;

      // Only update if the hash actually changed
      if (window.location.hash !== newHash) {
        history.replaceState(null, "", newHash);
        updateAriaCurrent();
      }
    }
  }

  function smoothScrollToHash(hash) {
    if (!hash?.startsWith("#")) return;
    document.getElementById(hash.slice(1))?.scrollIntoView({
      behavior: "smooth",
      block: "start",
    });
  }

  function handleAnchorClick(event) {
    const link = event.target.closest("a[href^='#']");
    if (!link) return;

    event.preventDefault();
    const href = link.getAttribute("href");
    smoothScrollToHash(href);
    if (href) {
      history.pushState(null, "", href);
      updateAriaCurrent();
    }
  }

  function updateAriaCurrent() {
    // Remove all aria-current attributes
    document.querySelectorAll(`${TOC_CONTAINER} ${TOC_LINK}`).forEach((link) => {
      link.setAttribute("aria-current", "false");
    });

    // Set aria-current based on current hash
    if (window.location.hash) {
      document
        .querySelector(`${TOC_CONTAINER} ${TOC_LINK}[href="${window.location.hash}"]`)
        ?.setAttribute("aria-current", "true");
    }
  }

  function initializeTableOfContents() {
    const tocContainer = document.querySelector(TOC_CONTAINER);
    if (!tocContainer) return;

    headingsArray = [...document.querySelectorAll(HEADINGS_SELECTOR)];

    // Cache positions after DOM is ready
    cacheHeadingPositions();

    // Recache on resize (debounced)
    let resizeTimeout;
    window.removeEventListener("resize", () => {
      clearTimeout(resizeTimeout);
      resizeTimeout = setTimeout(cacheHeadingPositions, 150);
    });
    window.addEventListener("resize", () => {
      clearTimeout(resizeTimeout);
      resizeTimeout = setTimeout(cacheHeadingPositions, 150);
    });

    window.removeEventListener("scroll", updateActiveSection);
    window.addEventListener("scroll", updateActiveSection);

    // Update aria-current on initial load
    updateAriaCurrent();

    updateActiveSection();
  }

  // Handle all anchor links on the page for smooth scrolling
  document.removeEventListener("click", handleAnchorClick);
  document.addEventListener("click", handleAnchorClick);

  // Handle initial hash on page load
  if (window.location.hash) {
    setTimeout(() => smoothScrollToHash(window.location.hash), 100);
  }

  initializeTableOfContents();

  new MutationObserver((mutations) => {
    if (mutations.some((m) => m.type === "childList" && m.addedNodes.length)) {
      setTimeout(initializeTableOfContents, 100);
    }
  }).observe(document.body, { childList: true, subtree: true });
})();
