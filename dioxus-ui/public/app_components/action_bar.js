document.querySelectorAll('[data-name="ActionBarButton"]').forEach((label) => {
  const radio = label.previousElementSibling; // radio is always before label

  const handleHover = (isHovering) => {
    if (!radio.checked) {
      const checkedLabel = document.querySelector(
        'input[name="action"]:checked',
      )?.nextElementSibling;

      if (isHovering) {
        checkedLabel?.style.setProperty("anchor-name", "none", "important");
        label.style.setProperty("anchor-name", "--action-bar-selected", "important");
      } else {
        label.style.removeProperty("anchor-name");
        checkedLabel?.style.removeProperty("anchor-name");
      }
    }
  };

  label.addEventListener("mouseenter", () => handleHover(true));
  label.addEventListener("mouseleave", () => handleHover(false));
  label.addEventListener("focus", () => handleHover(true));
  label.addEventListener("blur", () => handleHover(false));
});
