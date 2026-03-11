/**
 * SHIMMER FROM STRUCTURE
 * Auto-adapting skeleton loader - Vanilla JS implementation
 *
 * Usage (manual):
 *   shimmer(container, true)   // Enable shimmer
 *   shimmer(container, false)  // Disable shimmer
 *
 * Usage (automatic with Leptos):
 *   Container must have:
 *   - data-name="Shimmer"
 *   - data-shimmer-loading="true" or "false"
 *   MutationObserver will auto-trigger shimmer on attribute change.
 */

const LEAF_TAGS = ['img', 'svg', 'video', 'canvas', 'iframe', 'input', 'textarea', 'button'];

function isLeafElement(el) {
  const tag = el.tagName.toLowerCase();
  if (LEAF_TAGS.includes(tag)) return true;
  return el.children.length === 0;
}

function measureElements(el, parentRect) {
  const rect = el.getBoundingClientRect();

  // Skip zero-dimension elements
  if (rect.width === 0 || rect.height === 0) return [];

  if (isLeafElement(el)) {
    const style = getComputedStyle(el);
    const tag = el.tagName.toLowerCase();

    // Special handling for table cells - measure text width, not cell width
    let measureRect = rect;
    if ((tag === 'td' || tag === 'th') && el.childNodes.length > 0) {
      const hasOnlyText = [...el.childNodes].every(n => n.nodeType === Node.TEXT_NODE);
      if (hasOnlyText) {
        const range = document.createRange();
        range.selectNodeContents(el);
        measureRect = range.getBoundingClientRect();
      }
    }

    return [{
      x: measureRect.left - parentRect.left,
      y: measureRect.top - parentRect.top,
      w: measureRect.width,
      h: measureRect.height,
      r: style.borderRadius || '4px'
    }];
  }

  // Recurse into children
  return [...el.children].flatMap(child => measureElements(child, parentRect));
}

function createOverlay(elements) {
  const overlay = document.createElement('div');
  overlay.className = 'shimmer__overlay';

  elements.forEach(el => {
    // Use fallback radius for text (usually has 0px)
    const radius = el.r === '0px' ? '4px' : el.r;

    const block = document.createElement('div');
    block.className = 'shimmer__block';
    block.style.cssText = `
      left: ${el.x}px;
      top: ${el.y}px;
      width: ${el.w}px;
      height: ${el.h}px;
      border-radius: ${radius};
    `;

    const wave = document.createElement('div');
    wave.className = 'shimmer__wave';
    block.appendChild(wave);
    overlay.appendChild(block);
  });

  return overlay;
}

/**
 * Main shimmer function - exposed globally
 * @param {HTMLElement} container - The container element
 * @param {boolean} loading - Whether to show shimmer
 */
function shimmer(container, loading) {
  // Remove existing overlay
  const existingOverlay = container.querySelector(':scope > .shimmer__overlay');
  if (existingOverlay) existingOverlay.remove();

  // Toggle loading class
  container.classList.toggle('loading', loading);

  if (!loading) return;

  // Measure all leaf elements
  const parentRect = container.getBoundingClientRect();
  const elements = [...container.children]
    .filter(c => !c.classList.contains('shimmer__overlay'))
    .flatMap(c => measureElements(c, parentRect));

  // Create and append overlay
  const overlay = createOverlay(elements);
  container.appendChild(overlay);
}

// Expose shimmer globally
window.shimmer = shimmer;

// ============================================
// CSS Injection (for pages without inline CSS)
// ============================================
const style = document.createElement('style');
style.id = 'shimmer__styles';
style.textContent = `
  /* When loading, make text transparent but keep structure */
  .shimmer__container.loading *:not(.shimmer__overlay):not(.shimmer__overlay *) {
    color: transparent !important;
  }
  .shimmer__container.loading img:not(.shimmer__overlay *),
  .shimmer__container.loading svg:not(.shimmer__overlay *),
  .shimmer__container.loading video:not(.shimmer__overlay *) {
    opacity: 0;
  }

  /* Overlay positioning */
  .shimmer__overlay {
    position: absolute;
    inset: 0;
    z-index: 10;
    pointer-events: none;
    overflow: hidden;
  }

  /* Individual shimmer block
     Default: light gray for light backgrounds
     Dark mode: semi-transparent white (triggered by .dark class or data-mode="dark") */
  .shimmer__block {
    position: absolute;
    background: #e5e5e5;
    overflow: hidden;
  }

  /* Dark mode - only when app explicitly sets dark mode class */
  :root.dark .shimmer__block,
  :root[data-mode="dark"] .shimmer__block,
  :root[data-theme="dark"] .shimmer__block {
    background: rgba(255, 255, 255, 0.08);
  }

  /* The animated wave */
  .shimmer__wave {
    position: absolute;
    inset: 0;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.8), transparent);
    animation: shimmer__wave 1.5s infinite;
  }

  :root.dark .shimmer__wave,
  :root[data-mode="dark"] .shimmer__wave,
  :root[data-theme="dark"] .shimmer__wave {
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.15), transparent);
  }

  @keyframes shimmer__wave {
    0% { transform: translateX(-100%); }
    100% { transform: translateX(100%); }
  }
`;
if (!document.getElementById('shimmer__styles')) {
  document.head.appendChild(style);
}

// ============================================
// MutationObserver for Leptos integration
// ============================================
const processed = new WeakMap();

function processContainer(container) {
  const loading = container.dataset.shimmerLoading === 'true';
  const currentState = processed.get(container);
  const newState = loading ? 'on' : 'off';

  // Skip if already in this state (prevent infinite loops)
  if (currentState === newState) return;

  // Add shimmer__container class for CSS targeting
  if (!container.classList.contains('shimmer__container')) {
    container.classList.add('shimmer__container');
  }

  shimmer(container, loading);
  processed.set(container, newState);
}

function processAllContainers() {
  document.querySelectorAll('[data-name="Shimmer"]').forEach(processContainer);
}

// MutationObserver to watch for attribute changes
const observer = new MutationObserver(mutations => {
  for (const mutation of mutations) {
    // Watch for data-shimmer-loading attribute changes
    if (mutation.type === 'attributes' && mutation.attributeName === 'data-shimmer-loading') {
      const target = mutation.target;
      if (target.dataset?.name === 'Shimmer') {
        processContainer(target);
      }
    }
    // Watch for new shimmer containers added to DOM
    if (mutation.type === 'childList') {
      mutation.addedNodes.forEach(node => {
        if (node.nodeType === 1) {
          if (node.dataset?.name === 'Shimmer') {
            processContainer(node);
          }
          // Also check descendants
          node.querySelectorAll?.('[data-name="Shimmer"]').forEach(processContainer);
        }
      });
    }
  }
});

function startObserver() {
  observer.observe(document.body, {
    childList: true,
    subtree: true,
    attributes: true,
    attributeFilter: ['data-shimmer-loading']
  });
  // DON'T process containers on startup - let Leptos hydrate first
  // Shimmer will activate when user toggles the loading state
}

// Start observer when DOM is ready
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', startObserver);
} else {
  startObserver();
}
