use app_config::SiteConfig;
use leptos::prelude::*;
use leptos_meta::MetaTags;

use crate::app::App;

// CSS bundle URL with hash for cache-busting
// Hash is generated at build time from CSS content
const CSS_BUNDLE_URL: &str = concat!("/pkg/deploy_rust_ui.css?v=", env!("CSS_HASH"));

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        // <html lang="en" class="dark">
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta
                    name="viewport"
                    content="width=device-width, initial-scale=1.0, minimum-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover"
                />

                // NOTE: title, description, canonical, og:*, twitter:* are managed by SeoMeta component
                // to allow per-page customization without hydration mismatch

                // SEO metadata (static - not overridden by SeoMeta)
                <meta
                    name="robots"
                    content="index, follow, max-image-preview:large, max-snippet:-1, max-video-preview:-1"
                />
                <meta
                    name="keywords"
                    content="leptos ui components, rust ui components, leptos rust, tailwind css, rust ui library, cross-platform ui, rust fullstack, leptos components library"
                />

                // Basic metadata
                <meta name="author" content=SiteConfig::AUTHOR />
                <meta name="resource-type" content="document" />
                <meta name="mobile-web-app-capable" content="yes" />
                <meta name="apple-mobile-web-app-capable" content="yes" />
                <meta name="theme-color" content="#000000" />
                <meta name="apple-mobile-web-app-status-bar-style" content="black" />
                <meta name="msapplication-navbutton-color" content="#000000" />

                // Static OpenGraph tags (not overridden by SeoMeta)
                <meta property="og:site_name" content="Rust/UI" />
                <meta property="og:image:type" content="image/webp" />
                <meta
                    property="og:image:alt"
                    content="Leptos Rust UI Components · Cross-platform component registry"
                />
                <meta property="og:locale" content="en_US" />

                // Static Twitter Card tags (not overridden by SeoMeta)
                <meta name="twitter:site" content="@RustUI" />
                <meta name="twitter:creator" content="@RustUI" />
                <meta name="twitter:image:alt" content="Leptos Rust UI Components Library" />

                // Security policy headers
                <meta http-equiv="Permissions-Policy" content="payment=()" />

                // Preconnect to external domains
                <link rel="preconnect" href="https://github.com" crossorigin="anonymous" />
                <link rel="preconnect" href="https://www.linkedin.com" crossorigin="anonymous" />
                <link rel="preconnect" href="https://discord.gg" crossorigin="anonymous" />

                // Preconnect to Google Fonts (for /create font picker)
                <link rel="preconnect" href="https://fonts.googleapis.com" />
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
                // Load all theme fonts — async so they don't block rendering
                <link
                    rel="stylesheet"
                    href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Geist:wght@400;500;600;700&family=Roboto:wght@400;500;600;700&family=Noto+Sans:wght@400;500;600;700&family=DM+Sans:wght@400;500;600;700&family=Nunito+Sans:wght@400;500;600;700&family=Raleway:wght@400;500;600;700&family=Outfit:wght@400;500;600;700&family=Figtree:wght@400;500;600;700&family=Public+Sans:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500;600;700&family=Geist+Mono:wght@400;500;600;700&family=Lora:wght@400;600;700&family=Merriweather:wght@400;700&family=Playfair+Display:wght@400;600;700&family=Noto+Serif:wght@400;600;700&display=swap"
                    media="print"
                    onload="this.media='all'"
                />
                <noscript>
                    <link
                        rel="stylesheet"
                        href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Geist:wght@400;500;600;700&family=Roboto:wght@400;500;600;700&family=Noto+Sans:wght@400;500;600;700&family=DM+Sans:wght@400;500;600;700&family=Nunito+Sans:wght@400;500;600;700&family=Raleway:wght@400;500;600;700&family=Outfit:wght@400;500;600;700&family=Figtree:wght@400;500;600;700&family=Public+Sans:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500;600;700&family=Geist+Mono:wght@400;500;600;700&family=Lora:wght@400;600;700&family=Merriweather:wght@400;700&family=Playfair+Display:wght@400;600;700&family=Noto+Serif:wght@400;600;700&display=swap"
                    />
                </noscript>

                // Prevent dark mode flash - must run before page renders
                <script>
                    "if(localStorage.getItem('darkmode')==='true'||(localStorage.getItem('darkmode')===null&&window.matchMedia('(prefers-color-scheme:dark)').matches)){document.documentElement.classList.add('dark')}"
                </script>

                // Loading screen - injected via JS to avoid hydration mismatch (iOS only, home page)
                <style>
                    "html.loading-screen,html.loading-screen body{background:#18181b !important}"
                    "#app-loading-screen{position:fixed;top:-100px;right:0;bottom:-100px;left:0;z-index:9999;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:1.5rem;background:#18181b;clip-path:ellipse(150% 150% at 50% 0%);transition:clip-path 1s cubic-bezier(.4,0,.2,1)}"
                    "#app-loading-screen svg{width:1.5rem;height:1.5rem;color:#a1a1aa}"
                    "#app-loading-screen svg line{animation:ios-spin 1s linear infinite}"
                    "#app-loading-screen svg line:nth-child(1){animation-delay:0s}"
                    "#app-loading-screen svg line:nth-child(2){animation-delay:-0.875s}"
                    "#app-loading-screen svg line:nth-child(3){animation-delay:-0.75s}"
                    "#app-loading-screen svg line:nth-child(4){animation-delay:-0.625s}"
                    "#app-loading-screen svg line:nth-child(5){animation-delay:-0.5s}"
                    "#app-loading-screen svg line:nth-child(6){animation-delay:-0.375s}"
                    "#app-loading-screen svg line:nth-child(7){animation-delay:-0.25s}"
                    "#app-loading-screen svg line:nth-child(8){animation-delay:-0.125s}"
                    "@keyframes ios-spin{0%,39%,100%{opacity:0.2}40%{opacity:1}}"
                    "#app-loading-screen.fade-out{clip-path:ellipse(150% 0% at 50% 0%)}"
                    // fill-mode:backwards (not both/forwards): transform only exists during the
                    // 200ms animation. After it ends, the element reverts to its natural styles
                    // (no transform, no stacking context), so position:fixed children (Drawer,
                    // ContextMenu, Sonner) are unaffected. fill-mode:both would permanently apply
                    // the last keyframe's transform:translateY(0), breaking fixed element layout.
                    "@keyframes page__fade_in{from{opacity:0;transform:translateY(6px)}to{opacity:1}}"
                    ".page__fade{animation:page__fade_in 200ms ease-out backwards}"
                </style>
                <script>
                    "(function(){if(window.location.pathname!=='/') return;document.documentElement.classList.add('loading-screen');var l=document.createElement('div');l.id='app-loading-screen';l.innerHTML='<img src=\"/icons/logo-dark-square-88.png\" alt=\"\" width=\"88\" height=\"88\"><svg viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\"><line x1=\"12\" y1=\"2\" x2=\"12\" y2=\"6\"/><line x1=\"16.24\" y1=\"7.76\" x2=\"19.07\" y2=\"4.93\"/><line x1=\"18\" y1=\"12\" x2=\"22\" y2=\"12\"/><line x1=\"16.24\" y1=\"16.24\" x2=\"19.07\" y2=\"19.07\"/><line x1=\"12\" y1=\"18\" x2=\"12\" y2=\"22\"/><line x1=\"4.93\" y1=\"19.07\" x2=\"7.76\" y2=\"16.24\"/><line x1=\"2\" y1=\"12\" x2=\"6\" y2=\"12\"/><line x1=\"4.93\" y1=\"4.93\" x2=\"7.76\" y2=\"7.76\"/></svg>';document.documentElement.appendChild(l);document.addEventListener('DOMContentLoaded',function(){setTimeout(function(){document.documentElement.classList.remove('loading-screen');l.classList.add('fade-out');setTimeout(function(){l.remove()},1000)},1000)})})()"
                </script>

                // Favicon and PWA
                <link rel="icon" href="/favicon.ico" />
                <link rel="apple-touch-icon" href="/icons/apple-touch-icon.png" />
                <link rel="manifest" href="/manifest.json" />

                // RSS and JSON Feeds for content discovery
                <link rel="alternate" type="application/rss+xml" title="Rust/UI Changelog RSS Feed" href="/feed.xml" />
                <link
                    rel="alternate"
                    type="application/feed+json"
                    title="Rust/UI Changelog JSON Feed"
                    href="/feed.json"
                />

                // Preload critical CSS with fetchpriority and cache-busting hash
                <link rel="preload" href=CSS_BUNDLE_URL r#as="style" fetchpriority="high" />
                <link rel="stylesheet" href=CSS_BUNDLE_URL />

                // Preload and async load Sonner CSS (non-critical)
                <link rel="preload" href="/components/sonner.css" r#as="style" />
                <link rel="stylesheet" href="/components/sonner.css" media="print" onload="this.media='all'" />
                <noscript>
                    <link rel="stylesheet" href="/components/sonner.css" />
                </noscript>
                <link rel="preload" href="/components/vaul_drawer.css" r#as="style" />
                <link rel="stylesheet" href="/components/vaul_drawer.css" media="print" onload="this.media='all'" />

                // Load scripts (async for non-blocking parallel download, executes as soon as ready)
                <script async src="/components/resizable.bundle.js"></script>
                <script async src="/components/shimmer_init.js?v=3"></script>
                <script async src="/components/lazy_load_sonner.js"></script>
                <script async src="/components/sidenav.js?v=1"></script>

                // JSON-LD Structured Data for SEO (inlined at compile time — readable by AI crawlers)
                <script type="application/ld+json" inner_html=include_str!("../../public/schema.json")></script>

                // Plausible Analytics - Privacy-friendly web analytics
                <script defer data-domain="rust-ui.com" src="https://plausible.io/js/script.js"></script>

                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>

            <body>
                <App />
            </body>
        </html>
    }
}
