// Runs synchronously in <head>, before CSS/paint: prevents dark-mode flash and
// shows a branded loading screen on the home page until the app has mounted.
(function () {
    if (
        localStorage.getItem('darkmode') === 'true' ||
        (localStorage.getItem('darkmode') === null &&
            window.matchMedia('(prefers-color-scheme:dark)').matches)
    ) {
        document.documentElement.classList.add('dark');
    }

    if (window.location.pathname !== '/') return;

    var logoSrc = document.currentScript.dataset.logo;
    document.documentElement.classList.add('loading-screen');

    var overlay = document.createElement('div');
    overlay.id = 'app-loading-screen';
    overlay.innerHTML =
        '<img src="' + logoSrc + '" alt="" width="88" height="88">' +
        '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">' +
        '<line x1="12" y1="2" x2="12" y2="6"/><line x1="16.24" y1="7.76" x2="19.07" y2="4.93"/>' +
        '<line x1="18" y1="12" x2="22" y2="12"/><line x1="16.24" y1="16.24" x2="19.07" y2="19.07"/>' +
        '<line x1="12" y1="18" x2="12" y2="22"/><line x1="4.93" y1="19.07" x2="7.76" y2="16.24"/>' +
        '<line x1="2" y1="12" x2="6" y2="12"/><line x1="4.93" y1="4.93" x2="7.76" y2="7.76"/></svg>';
    document.documentElement.appendChild(overlay);

    function dismiss() {
        setTimeout(function () {
            document.documentElement.classList.remove('loading-screen');
            overlay.classList.add('fade-out');
            setTimeout(function () {
                overlay.remove();
            }, 1000);
        }, 1000);
    }

    // On native builds (dx serve --platform ios/android/desktop) the DOM is
    // assembled client-side, so by the time this externally-loaded script runs,
    // DOMContentLoaded has often already fired — an event listener for it would
    // never trigger, leaving the overlay stuck forever. Web SSR delivers this
    // script early enough that readyState is still 'loading'.
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', dismiss);
    } else {
        dismiss();
    }
})();
