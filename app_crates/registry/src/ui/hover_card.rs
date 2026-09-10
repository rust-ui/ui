use std::sync::atomic::{AtomicU64, Ordering};

use dioxus::prelude::*;
use tw_merge::tw_merge;

static HOVER_CARD_COUNTER: AtomicU64 = AtomicU64::new(0);

fn use_hover_card_id() -> String {
    use_hook(|| {
        let id = HOVER_CARD_COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("{id}")
    })
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum HoverCardSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

#[derive(Clone)]
struct HoverCardContext {
    anchor_name: String,
    trigger_id: String,
    content_id: String,
}

#[component]
pub fn HoverCard(children: Element, #[props(default = HoverCardSide::Bottom)] side: HoverCardSide) -> Element {
    let id = use_hover_card_id();
    let anchor_name = format!("--hc_anchor_{id}");
    let trigger_id = format!("hc_trigger_{id}");
    let content_id = format!("hc_content_{id}");

    let (position_styles, transform_origin) = match side {
        HoverCardSide::Bottom => ("position-area: block-end; margin-top: 8px;".to_string(), "center top".to_string()),
        HoverCardSide::Top => {
            ("position-area: block-start; margin-bottom: 8px;".to_string(), "center bottom".to_string())
        }
        HoverCardSide::Left => {
            ("position-area: inline-start; margin-right: 8px;".to_string(), "right center".to_string())
        }
        HoverCardSide::Right => ("position-area: inline-end; margin-left: 8px;".to_string(), "left center".to_string()),
    };

    let css = format!(
        "#{content_id} {{
            position-anchor: {anchor_name};
            inset: auto;
            {position_styles}
            position-try-fallbacks: flip-block;
            position-try-order: most-height;
            position-visibility: anchors-visible;
        }}
        #{content_id}:popover-open {{
            opacity: 1;
            transform: scale(1) translateY(0px);
        }}
        @starting-style {{
            #{content_id}:popover-open {{
                opacity: 0;
                transform: scale(0.95) translateY(-4px);
            }}
        }}
        #{content_id} {{
            transition:
                display 0.2s allow-discrete,
                overlay 0.2s allow-discrete,
                transform 0.2s cubic-bezier(0.16, 1, 0.3, 1),
                opacity 0.15s ease-out;
            opacity: 0;
            transform: scale(0.95) translateY(-4px);
            transform-origin: {transform_origin};
        }}"
    );

    let script = format!(
        r#"(function() {{
            const setup = () => {{
                const trigger = document.getElementById('{trigger_id}');
                const content = document.getElementById('{content_id}');
                if (!trigger || !content) {{ setTimeout(setup, 50); return; }}
                if (trigger.dataset.hcInit) return;
                trigger.dataset.hcInit = '1';
                let t;
                const show = () => {{ clearTimeout(t); t = setTimeout(() => {{ try {{ content.showPopover(); }} catch(e) {{}} }}, 150); }};
                const hide = () => {{ clearTimeout(t); t = setTimeout(() => {{ try {{ content.hidePopover(); }} catch(e) {{}} }}, 150); }};
                trigger.addEventListener('mouseenter', show);
                trigger.addEventListener('mouseleave', hide);
                trigger.addEventListener('focus', show);
                trigger.addEventListener('blur', hide);
                content.addEventListener('mouseenter', () => clearTimeout(t));
                content.addEventListener('mouseleave', hide);
            }};
            if (document.readyState === 'loading') {{
                document.addEventListener('DOMContentLoaded', setup);
            }} else {{
                setup();
            }}
        }})();"#
    );

    provide_context(HoverCardContext {
        anchor_name: anchor_name.clone(),
        trigger_id: trigger_id.clone(),
        content_id: content_id.clone(),
    });

    rsx! {
        style { "{css}" }
        {children}
        script { "{script}" }
    }
}

#[component]
pub fn HoverCardTrigger(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let ctx = use_context::<HoverCardContext>();
    let c = tw_merge!("inline-block", class.as_deref().unwrap_or(""));
    let anchor_style = format!("anchor-name: {}", ctx.anchor_name);

    rsx! {
        span {
            id: "{ctx.trigger_id}",
            class: "{c}",
            style: "{anchor_style}",
            {children}
        }
    }
}

#[component]
pub fn HoverCardContent(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let ctx = use_context::<HoverCardContext>();
    let c = tw_merge!(
        "overflow-visible relative z-50 p-4 rounded-lg border bg-card shadow-md w-64",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div {
            id: "{ctx.content_id}",
            class: "{c}",
            popover: "manual",
            {children}
        }
    }
}
