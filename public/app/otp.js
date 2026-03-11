(function () {
  function initContainer(root) {
    if (root.dataset.otpInit) return;
    root.dataset.otpInit = '1';

    var input = root.querySelector('input[data-otp-input]');
    if (!input) return;

    var maxLen = parseInt(input.getAttribute('maxlength') || '6', 10);

    function getSlots() {
      return Array.from(root.querySelectorAll('[data-otp-slot]')).sort(function (a, b) {
        return parseInt(a.dataset.otpIndex, 10) - parseInt(b.dataset.otpIndex, 10);
      });
    }

    function update() {
      var val = input.value;
      var focused = document.activeElement === input;
      var sel = focused ? (input.selectionStart || 0) : -1;

      getSlots().forEach(function (slot) {
        var idx = parseInt(slot.dataset.otpIndex, 10);
        var char = val[idx] || '';

        // Active = cursor is at this slot position (or at end when value not full)
        var isActive = focused && (
          sel === idx ||
          (sel >= val.length && idx === val.length && val.length < maxLen)
        );

        // Char display — span is pre-rendered by SSR, just update textContent
        var charEl = slot.querySelector('[data-otp-char]');
        if (charEl) charEl.textContent = char;

        // Active styling
        slot.dataset.active = isActive ? 'true' : 'false';

        // Caret visibility
        var caret = slot.querySelector('[data-otp-caret]');
        if (caret) {
          caret.style.display = (isActive && !char) ? 'flex' : 'none';
        }
      });
    }

    // Click on slot → focus input
    getSlots().forEach(function (slot) {
      slot.addEventListener('click', function () {
        if (!input.disabled) input.focus();
      });
    });

    // Digits only
    input.addEventListener('beforeinput', function (e) {
      if (e.inputType === 'insertText' && e.data && !/^\d+$/.test(e.data)) {
        e.preventDefault();
      }
    });

    input.addEventListener('input', update);

    input.addEventListener('keydown', function () {
      setTimeout(update, 0);
    });

    input.addEventListener('focus', function () {
      setTimeout(function () {
        var len = input.value.length;
        input.setSelectionRange(len, len);
        update();
      }, 0);
    });

    input.addEventListener('blur', update);

    update();
  }

  function initAll() {
    document.querySelectorAll('[data-otp-root]').forEach(initContainer);
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initAll);
  } else {
    initAll();
  }

  // Watch for dynamically added OTP components (e.g. after Leptos hydration)
  new MutationObserver(function (mutations) {
    mutations.forEach(function (m) {
      m.addedNodes.forEach(function (node) {
        if (node.nodeType !== 1) return;
        if (node.matches && node.matches('[data-otp-root]')) initContainer(node);
        if (node.querySelectorAll) node.querySelectorAll('[data-otp-root]').forEach(initContainer);
      });
    });
  }).observe(document.body, { childList: true, subtree: true });
})();
