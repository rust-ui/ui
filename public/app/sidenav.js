(function () {
  document.addEventListener("keydown", function (e) {
    // Ctrl+B (Windows/Linux) or Cmd+B (Mac)
    if (!(e.key === "b" && (e.ctrlKey || e.metaKey))) return;

    // Skip if an editable element is focused (INPUT, TEXTAREA, contenteditable)
    const el = document.activeElement;
    if (
      el &&
      (el.tagName === "INPUT" ||
        el.tagName === "TEXTAREA" ||
        el.isContentEditable)
    )
      return;

    // Prefer clicking the trigger button — routes through Leptos signal when SidenavWrapper context is present
    const trigger = document.querySelector('[data-name="SidenavTrigger"]');
    if (trigger) {
      e.preventDefault();
      trigger.click();
      return;
    }

    // Fallback: direct DOM toggle (no SidenavTrigger in DOM)
    const sidenav = document.querySelector('[data-name="Sidenav"]');
    if (!sidenav || !sidenav.hasAttribute("data-sidenav")) return;
    e.preventDefault();
    const current = sidenav.getAttribute("data-state");
    sidenav.setAttribute(
      "data-state",
      current === "Collapsed" ? "Expanded" : "Collapsed",
    );
  });
})();
