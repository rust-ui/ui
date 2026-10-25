+++
title = "Dark Mode in Dioxus with Tailwind CSS: Design Tokens Guide"
description = "Add dark mode to a Dioxus app with Tailwind CSS: CSS variable tokens, a theme toggle, system preference, and no flash of the wrong theme. Code included."
category = "Components"
publish_date = "2026-10-25"
last_updated = "2026-10-25"
author = "Max Wells"
author_role = "Creator of Rust/UI and Rustify"
author_image = "/articles/author-max-wells.webp"
short_title = "Dioxus dark mode"
keywords = ["Dioxus dark mode", "Tailwind dark mode Rust", "Dioxus theming", "CSS variables design tokens", "Dioxus theme toggle", "Rust UI dark theme"]
+++

The reliable way to add dark mode to a Dioxus app is to define your colors as CSS variables, switch them with a `dark` class on the `<html>` element, and style every component with token classes such as `bg-background` and `text-foreground`. Components then never mention a theme, and one class flip restyles the whole app.

**TL;DR:** Do not write `dark:` variants on every element. Define semantic tokens (`--background`, `--card`, `--primary`) once in `tailwind.css`, override them under `.dark`, and use token utilities everywhere. Read the saved choice or the system preference before first paint to avoid a flash, persist the choice, and keep the toggle a small component.

- **Tokens:** roles, not palette colors, so a rebrand edits one file.
- **Switch:** a `dark` class on `<html>`, which Tailwind's `dark:` variant and your variables both follow.
- **No flash:** set the class from a tiny inline script before the app renders.
- **Persistence:** `localStorage` for the choice, `prefers-color-scheme` as the fallback.
- **Ready-made:** the Rust/UI [Theme Toggle](/docs/components/theme-toggle) implements all of this.

---

## Who is this guide for?

This guide is for Dioxus developers who already style with Tailwind and now want light and dark themes that do not need per-component fixes. You should have Tailwind running (Dioxus 0.7 detects `tailwind.css`). If layout is still shaky, read [responsive Rust UI with Dioxus and Tailwind](/articles/responsive-rust-ui-with-dioxus-tailwind) first. Jump to tokens for the CSS, to the toggle for the component, and to the flash section if your page blinks white on load.

---

## What is the right way to structure dark mode?

**Separate what a color means from what it looks like.** A button background is `primary`; whether primary is dark blue or light blue is a theme decision. If components only know the meaning, the theme can change without touching them.

| Approach | Dark mode effort | Rebrand effort | Risk |
|---|---|---|---|
| `dark:` variant on every element | Edit every component | Edit every component | Missed elements, inconsistent contrast |
| Semantic tokens via CSS variables | One CSS block | One CSS block | Needs discipline to avoid raw colors |
| Separate stylesheet per theme | Duplicate the sheet | Duplicate the sheet | Drift between themes |

> **Bottom line:** Tokens turn dark mode from a per-component chore into a single CSS block. Use raw palette colors only for one-off illustrations.

![Theme flow: stored choice or system preference decides the dark class on the html element, which switches CSS variable values used by Tailwind tokens](/articles/dioxus-dark-mode-tailwind-theming-asset-1.svg)

*The saved choice or the system preference decides one class on `<html>`. That class swaps the variable values, and token classes follow automatically.*

---

## How do you define design tokens in tailwind.css?

**Declare variables on `:root`, override them under `.dark`, and map them to Tailwind colors.** With Tailwind 4 you expose them through `@theme`; the shape is the same on older setups.

```css
@import "tailwindcss";

@custom-variant dark (&:where(.dark, .dark *));

:root {
  --background: oklch(1 0 0);
  --foreground: oklch(0.15 0 0);
  --card: oklch(0.98 0 0);
  --muted-foreground: oklch(0.45 0 0);
  --border: oklch(0.9 0 0);
  --primary: oklch(0.55 0.2 260);
  --primary-foreground: oklch(0.98 0 0);
}

.dark {
  --background: oklch(0.14 0 0);
  --foreground: oklch(0.97 0 0);
  --card: oklch(0.18 0 0);
  --muted-foreground: oklch(0.7 0 0);
  --border: oklch(0.28 0 0);
  --primary: oklch(0.7 0.16 260);
  --primary-foreground: oklch(0.15 0 0);
}

@theme inline {
  --color-background: var(--background);
  --color-foreground: var(--foreground);
  --color-card: var(--card);
  --color-muted-foreground: var(--muted-foreground);
  --color-border: var(--border);
  --color-primary: var(--primary);
  --color-primary-foreground: var(--primary-foreground);
}
```

The `@custom-variant` line makes Tailwind's `dark:` variant follow the `.dark` class instead of the OS setting, so manual toggling and the `dark:` escape hatch agree. After this, a component writes `bg-card text-foreground border-border` and is correct in both themes.

```rust
rsx! {
    section { class: "rounded-2xl border border-border bg-card p-6 text-foreground",
        h2 { class: "text-lg font-semibold", "Account" }
        p { class: "mt-1 text-sm text-muted-foreground", "Works in light and dark with no dark: classes." }
    }
}
```

A token system also changes how you review code. A pull request that introduces `bg-zinc-900` or `text-[#333]` in a component is a signal to stop and ask which role that color plays, and to add a token if the role is genuinely new. Over time this keeps the palette small, which is what makes the product feel designed rather than assembled. Choose dark values by contrast, not by inverting. Pure black backgrounds with pure white text cause glare; slightly lifted surfaces (`card` lighter than `background`) communicate elevation, which shadows no longer do on dark backgrounds. Aim for at least 4.5:1 contrast for body text in both themes.

---

## How do you build the theme toggle?

**Toggle the `dark` class on the root element and persist the choice.** In a Dioxus web app the DOM work goes through `document::eval`, and the component stays tiny.

```rust
use dioxus::prelude::*;

#[component]
pub fn ThemeToggle() -> Element {
    let mut dark = use_signal(|| false);

    // Read the real state once the component mounts in the browser.
    use_effect(move || {
        spawn(async move {
            let mut eval = document::eval(
                "dioxus.send(document.documentElement.classList.contains('dark'))",
            );
            if let Ok(is_dark) = eval.recv::<bool>().await {
                dark.set(is_dark);
            }
        });
    });

    let toggle = move |_| {
        let next = !dark();
        dark.set(next);
        document::eval(&format!(
            "document.documentElement.classList.toggle('dark', {next}); \
             localStorage.setItem('theme', '{}');",
            if next { "dark" } else { "light" }
        ));
    };

    rsx! {
        button {
            r#type: "button",
            class: "rounded-md border border-border p-2 hover:bg-accent",
            aria_label: if dark() { "Switch to light theme" } else { "Switch to dark theme" },
            onclick: toggle,
            if dark() { "Light" } else { "Dark" }
        }
    }
}
```

Note the accessible name changes with the state, so a screen reader announces the action, not just "button". If you want icons and a polished animation, the Rust/UI [Theme Toggle](/docs/components/theme-toggle) is the finished version: it reads `localStorage`, falls back to the system preference, flips the `.dark` class, and persists the choice. On desktop and mobile targets the same class flip works because the UI runs in a webview, which fits the approach in [shipping a cross-platform Rust UI](/articles/ship-cross-platform-rust-ui-dioxus).

---

## How do you avoid the flash of the wrong theme?

**Set the class before the first paint, not after the app mounts.** If the theme is applied in a component effect, a dark-mode user sees a white page for a moment on every load.

Add a small inline script to the document head that runs before rendering:

```html
<script>
  (function () {
    try {
      var saved = localStorage.getItem('theme');
      var system = window.matchMedia('(prefers-color-scheme: dark)').matches;
      if (saved === 'dark' || (!saved && system)) {
        document.documentElement.classList.add('dark');
      }
    } catch (e) {}
  })();
</script>
```

It is synchronous on purpose. It reads the stored choice first, falls back to the system preference, and silently ignores storage errors such as a blocked `localStorage`. Because it runs before the body paints, the first frame is already correct. Add `color-scheme: dark` on `.dark` (and `light` on `:root`) so native scrollbars, form controls, and the canvas color match.

| Symptom | Likely cause | Fix |
|---|---|---|
| White flash on dark load | Theme applied after mount | Inline script in `<head>` |
| Toggle works, reload resets | Choice not persisted | Write `localStorage` on toggle |
| Dark OS, light site | Only `dark:` media variant used | Use `@custom-variant` and the class |
| Native inputs stay light | Missing `color-scheme` | Set `color-scheme` per theme |
| Text unreadable in one theme | Raw palette color | Replace with a token |

---

## How do you support three modes: light, dark, and system?

**Store a tri-state, resolve it at load, and listen for OS changes.** Many users want "follow my system", not a fixed theme.

```rust
#[derive(Clone, Copy, PartialEq, Default)]
enum ThemeChoice {
    #[default]
    System,
    Light,
    Dark,
}

impl ThemeChoice {
    fn as_str(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}
```

Store `as_str()` in `localStorage`. At load, resolve `system` through `matchMedia`, and add a `change` listener on the media query so the page follows the OS if the user switches at sunset. Show the control as a three-option group, for which a [Toggle Group](/docs/components/toggle-group) fits well, and mark the active option with `aria-pressed`.

---

## How do you handle images, charts, and code in dark mode?

**Treat anything with a baked-in color as part of the theme.** Tokens fix your UI, but content with fixed colors can break it.

- **SVG icons:** use `currentColor` for fill and stroke so they inherit text color.
- **Diagrams and screenshots:** provide a dark variant, or give the image a light card background so it stays legible.
- **Charts:** build series colors from tokens, and check contrast on both backgrounds.
- **Syntax highlighting:** pick a code theme per mode, or use one dark code theme in both, which is a common and acceptable choice.
- **Shadows:** reduce or replace them with a border or a lighter surface on dark.

---

## What belongs in a theme and what does not?

**Put color, radius, and shadow in the theme. Keep layout and copy out of it.** If switching a theme moves elements around, the theme has taken on a job that belongs to the components, and testing gets much harder.

---

## How do you test a theme before users do?

**Check contrast, states, and edge surfaces in both themes, not just the home page.** A theme usually breaks in the corners: disabled buttons, focus rings, selected rows, error text, and tooltips.

1. Build a single "theme gallery" route that renders every component in every state. Flip the toggle and scan it once instead of hunting through the app.
2. Run a contrast check on text and on interactive borders. Muted text on a card is the usual failure, since `muted-foreground` on `card` can drop below 4.5:1 in dark.
3. Tab through the page and look at the focus ring in both themes. A ring that is visible on white can disappear on a dark surface.
4. Print preview and forced-colors mode (Windows high contrast) are worth one pass if your audience includes enterprise users.
5. Reload with dark selected, with light selected, and with nothing stored. All three must look right on the first frame.

Automated screenshots catch regressions over time. Capture the gallery in each theme in CI and diff them, so a changed token is a deliberate review, not a surprise. Theme bugs are rarely logic bugs; they are one wrong variable, and a visual diff finds that variable in seconds.

---

## When should you add more than two themes?

**Add themes only when they carry real value, and keep them as token sets.** Brand accents, a high-contrast mode, or a "sepia" reading theme are all legitimate. Each one is another block of variable overrides, applied by a class or a `data-theme` attribute:

```css
[data-theme="ocean"] {
  --primary: oklch(0.6 0.15 220);
  --primary-foreground: oklch(0.98 0 0);
}
```

Because components only reference tokens, a new theme costs one CSS block and zero component edits. This is also how teams ship white-label products: each customer gets a token block with their brand color, and the component code is identical across all of them. Keep the token names stable and treat them as a public API, because renaming `--card` later touches every theme you have shipped. That is the payoff of the whole approach. The risk is combinatorial: three accents times two modes is six combinations to check, so keep the number small and test each pair in your gallery. Also keep the stored value validated; if a saved theme name no longer exists, fall back to the default instead of rendering unstyled.

---

## Why does dark mode matter beyond looks?

**It is an accessibility and comfort feature, and users now expect it.** Many people read in dim rooms, use screens for hours, or have light sensitivity, and a bright white page is a real cost for them. Following the system preference respects a choice the user already made once, at the OS level, so they do not have to make it again for every site. It also affects perceived quality: an app that honors the setting looks finished, while one that ignores it looks dated. Because the token approach costs almost nothing once set up, there is little reason to skip it, and the same variables later support high-contrast or brand themes with no extra component work.

---

## What are the common dark mode mistakes?

| Mistake | Symptom | Fix |
|---|---|---|
| Pure `#000` and `#fff` | Eye strain, harsh edges | Slightly softened surface and text |
| Inverting colors mechanically | Muddy, low-contrast UI | Hand-pick dark values by contrast |
| `dark:` on every element | Hundreds of edits per rebrand | Semantic tokens |
| Theme set after mount | Flash of white | Inline head script |
| Ignoring the system preference | Surprises users | Default to `prefers-color-scheme` |
| Forgetting form controls | Light inputs in dark pages | `color-scheme` plus tokenized inputs |
| No accessible name on toggle | Unclear to screen readers | Dynamic `aria-label` |

Once your tokens are in place, every new component inherits dark mode for free. If you copy components from the registry, they already use these token names, so the [installation guide](/docs/installation) walks through adding them to an existing project, and designing your own with [typed props](/articles/reusable-dioxus-components-typed-props) keeps theming out of the API.

---

## Frequently Asked Questions

### How do I enable dark mode in Dioxus?

Define color tokens as CSS variables, override them under a `.dark` class, and toggle that class on the `<html>` element. Style components with token classes such as `bg-background`, so no component needs theme logic.

### Does Tailwind CSS dark mode work with Dioxus?

Yes. Dioxus builds your `tailwind.css`, and the `dark:` variant works as usual. Use a class-based custom variant so the `.dark` class, not only the OS setting, controls it.

### How do I stop the white flash on page load?

Run a small inline script in the document head that adds the `dark` class from `localStorage` or the system preference before the first paint, instead of doing it in a component effect.

### How do I follow the system theme in Dioxus?

Read `window.matchMedia('(prefers-color-scheme: dark)')` when no saved choice exists, and add a `change` listener so the app follows OS changes while the user keeps the system option.

### Where should I store the theme choice?

Use `localStorage` on the web. It is a per-device preference, not sensitive data. Wrap access in a try and catch, because storage can be blocked in private windows.

### Does dark mode work on Dioxus desktop and mobile?

Yes. Those targets render in a webview, so the same CSS variables and `.dark` class apply. Persist the choice through your platform storage layer if `localStorage` is not suitable.

### Should I use pure black for the dark background?

Usually no. A very dark gray with slightly lighter card surfaces reads better, reduces glare, and shows elevation. Check that body text keeps at least a 4.5:1 contrast ratio.

### Is there a ready-made theme toggle for Dioxus?

Yes. Rust/UI includes a [Theme Toggle](/docs/components/theme-toggle) with animated icons that reads the saved or system preference, flips the class, and persists the choice.

---

## Sources

- [Tailwind CSS: dark mode](https://tailwindcss.com/docs/dark-mode)
- [MDN: prefers-color-scheme](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-color-scheme)
- [MDN: color-scheme](https://developer.mozilla.org/en-US/docs/Web/CSS/color-scheme)
- [WCAG 2.2: contrast minimum](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html)
