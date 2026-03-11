// * Event delegation for all carousel button clicks
document.addEventListener("click", (e) => {
  const btn = e.target.closest('[data-name="CardCarouselNavButton"]');
  if (!btn) return;

  // Allows to wrap in a `a` tag and prevent the navigation when clicking NavButton.
  e.stopPropagation();
  e.preventDefault();

  const root = btn.closest('[data-name="CardCarousel"]');
  const carousel = root.querySelector('[data-name="CardCarouselTrack"]');
  const buttons = root.querySelectorAll('[data-name="CardCarouselNavButton"]');
  const isPrev = btn === buttons[0];

  carousel.scrollBy({ left: carousel.clientWidth * (isPrev ? -1 : 1) });
});

// * Event delegation for all carousel scroll events (using capture phase)
document.addEventListener(
  "scroll",
  (e) => {
    const carousel = e.target.closest('[data-name="CardCarouselTrack"]');
    if (!carousel) return;

    const root = carousel.closest('[data-name="CardCarousel"]');
    const indicators = root.querySelectorAll('[data-name="CardCarouselIndicator"]');
    const buttons = root.querySelectorAll('[data-name="CardCarouselNavButton"]');

    const index = Math.round(carousel.scrollLeft / carousel.clientWidth);
    indicators.forEach((dot, i) => dot.toggleAttribute("aria-current", i === index));
    buttons[0].toggleAttribute("aria-disabled", index === 0);
    buttons[1].toggleAttribute("aria-disabled", index === indicators.length - 1);
  },
  true,
);
