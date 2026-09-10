use dioxus::html::geometry::WheelDelta;
use dioxus::html::input_data::MouseButton;
use dioxus::html::input_data::keyboard_types::{Key, Modifiers};
use dioxus::prelude::*;
use icons::{Clipboard, Copy, Pencil, Plus, SquareDashedMousePointer, Trash2};
use tw_merge::tw_merge;

use crate::hooks::use_workflow::{WorkflowNode, WorkflowState};
use crate::ui::context_menu::{ContextMenuGroup, ContextMenuLabel};

/// Passed through component context so WorkflowNodeWrapper can open the node
/// context menu without needing an extra prop (would break every existing demo).
#[derive(Clone, Copy)]
struct NodeCmCtx(Signal<Option<(usize, f64, f64)>>);

const MENU_BTN: &str = "inline-flex items-center gap-2 w-full rounded-sm px-2 py-1.5 text-sm transition-colors hover:bg-accent cursor-pointer";
const MENU_BTN_DANGER: &str = "inline-flex items-center gap-2 w-full rounded-sm px-2 py-1.5 text-sm transition-colors hover:bg-destructive/10 text-destructive cursor-pointer";

const VIEWPORT_W: f64 = 800.0;
const VIEWPORT_H: f64 = 450.0;
const MINI_W: f64 = 140.0;
const MINI_H: f64 = 90.0;
const WORLD_REF_W: f64 = 1600.0;
const WORLD_REF_H: f64 = 900.0;

pub const NODE_H: f64 = 72.0;

// ── Node primitives ───────────────────────────────────────────────────────────

#[component]
pub fn WfNode(
    #[props(optional)] target: bool,
    #[props(optional)] source: bool,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "relative w-full rounded-md border bg-card shadow-sm text-card-foreground",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div { class: "{merged}",
            if target {
                div {
                    class: "absolute left-0 top-1/2 size-3 rounded-full border-2 border-primary bg-background pointer-events-none",
                    style: "transform: translate(-50%, -50%);",
                }
            }
            if source {
                div {
                    class: "absolute right-0 top-1/2 size-3 rounded-full border-2 border-primary bg-background pointer-events-none",
                    style: "transform: translate(50%, -50%);",
                }
            }
            {children}
        }
    }
}

#[component]
pub fn WfNodeHeader(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "flex items-center gap-2 px-3 py-2.5 border-b bg-secondary/50 rounded-t-md",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { class: "{merged}", {children} } }
}

#[component]
pub fn WfNodeTitle(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("text-xs font-medium", class.as_deref().unwrap_or(""));
    rsx! { span { class: "{merged}", {children} } }
}

#[component]
pub fn WfNodeDescription(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("text-[10px] text-muted-foreground", class.as_deref().unwrap_or(""));
    rsx! { p { class: "{merged}", {children} } }
}

#[component]
pub fn WfNodeContent(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("px-3 py-2", class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}", {children} } }
}

#[component]
pub fn WfNodeFooter(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "flex items-center gap-2 px-3 py-2.5 border-t bg-secondary/50 rounded-b-md",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { class: "{merged}", {children} } }
}

// ── WorkflowCanvas ────────────────────────────────────────────────────────────
//
// Viewport div (overflow:hidden, fixed height)
// └── world div (3000×2000, CSS transform: translate+scale)
//     ├── dot background
//     ├── SVG bezier edges
//     └── children (WorkflowNode instances)
// └── overlay (viewport space — WorkflowControls, WorkflowMinimap, etc.)

#[component]
pub fn WorkflowCanvas(state: WorkflowState, children: Element, #[props(optional)] overlay: Option<Element>) -> Element {
    let is_busy = state.is_dragging() || state.is_panning();
    let is_rubber_banding = state.rubber_band.read().is_some();
    let locked = state.is_locked();
    let edge_paths = state.edge_paths(NODE_H);
    let selected_ei = state.selected_edge_idx();
    let transform = state.world_transform();
    let mut state = state;
    let mut canvas_origin = use_signal(|| (0.0_f64, 0.0_f64));
    let mut node_cm: Signal<Option<(usize, f64, f64)>> = use_signal(|| None);
    let mut edge_cm: Signal<Option<(usize, f64, f64)>> = use_signal(|| None);
    let mut canvas_cm: Signal<Option<(f64, f64)>> = use_signal(|| None);
    let mut edge_edit_vp_pos: Signal<Option<(f64, f64)>> = use_signal(|| None);
    provide_context(NodeCmCtx(node_cm));
    let rubber_band_rect = (*state.rubber_band.read()).map(|rb| {
        let (ox, oy) = *canvas_origin.read();
        let x = rb.start_x.min(rb.cur_x) - ox;
        let y = rb.start_y.min(rb.cur_y) - oy;
        let w = (rb.cur_x - rb.start_x).abs();
        let h = (rb.cur_y - rb.start_y).abs();
        (x, y, w, h)
    });

    rsx! {
        div {
            class: "relative rounded-lg border bg-background overflow-hidden select-none outline-none",
            style: format!(
                "height: 450px; cursor: {}; touch-action: none;",
                if locked { "default" } else if is_rubber_banding { "crosshair" } else if is_busy { "grabbing" } else { "grab" }
            ),
            tabindex: "0",
            oncontextmenu: move |ev| {
                ev.prevent_default();
                let c = ev.data().client_coordinates();
                canvas_cm.set(Some((c.x, c.y)));
            },

            onmounted: move |data| {
                spawn(async move {
                    if let Ok(rect) = data.get_client_rect().await {
                        canvas_origin.set((rect.min_x(), rect.min_y()));
                    }
                });
            },

            onkeydown: move |ev| {
                if locked { return; }
                let data = ev.data();
                let mods = data.modifiers();
                let ctrl = mods.contains(Modifiers::CONTROL) || mods.contains(Modifiers::META);
                let shift = mods.contains(Modifiers::SHIFT);
                match data.key() {
                    Key::Character(ref k) if ctrl && k == "z" && !shift => {
                        ev.prevent_default();
                        state.undo();
                    }
                    Key::Character(ref k) if ctrl && (k == "y" || (k == "z" && shift)) => {
                        ev.prevent_default();
                        state.redo();
                    }
                    Key::Character(ref k) if ctrl && k == "c" => {
                        ev.prevent_default();
                        state.copy_selected();
                    }
                    Key::Character(ref k) if ctrl && k == "v" => {
                        ev.prevent_default();
                        state.paste_nodes();
                    }
                    Key::Character(ref k) if ctrl && k == "d" => {
                        ev.prevent_default();
                        state.duplicate_selected();
                    }
                    Key::Delete | Key::Backspace => {
                        ev.prevent_default();
                        state.delete_selected();
                        state.delete_selected_edge();
                    }
                    Key::Escape => {
                        state.deselect();
                        state.cancel_edit();
                        state.cancel_edit_edge();
                        edge_edit_vp_pos.set(None);
                    }
                    Key::ArrowUp    => { ev.prevent_default(); state.nudge_selected(0.0, -20.0); }
                    Key::ArrowDown  => { ev.prevent_default(); state.nudge_selected(0.0,  20.0); }
                    Key::ArrowLeft  => { ev.prevent_default(); state.nudge_selected(-20.0, 0.0); }
                    Key::ArrowRight => { ev.prevent_default(); state.nudge_selected( 20.0, 0.0); }
                    _ => {}
                }
            },

            onmousedown: move |ev| {
                if locked { return; }
                let data = ev.data();
                if data.trigger_button() == Some(MouseButton::Secondary) { return; }
                let c = data.client_coordinates();
                let shift = data.modifiers().contains(Modifiers::SHIFT);
                state.finish_edit();
                if state.is_connecting() {
                    state.cancel_connect();
                } else if shift {
                    state.start_rubber_band(c.x, c.y);
                } else {
                    state.deselect();
                    state.start_pan(c.x, c.y);
                }
            },
            onmousemove: move |ev| {
                if locked { return; }
                let c = ev.data().client_coordinates();
                state.update_drag(c.x, c.y);
                state.update_pan(c.x, c.y);
                state.update_rubber_band(c.x, c.y);
                let ec = ev.data().element_coordinates();
                state.update_connect_mouse(ec.x, ec.y);
            },
            onmouseup: move |_| {
                state.stop_drag();
                state.stop_pan();
                state.cancel_connect();
                state.finish_rubber_band(NODE_H, *canvas_origin.read());
            },
            onmouseleave: move |_| {
                state.stop_drag();
                state.stop_pan();
                state.cancel_connect();
                state.cancel_rubber_band();
            },
            onwheel: move |ev| {
                if locked { return; }
                ev.prevent_default();
                let pos = ev.data().element_coordinates();
                let delta_y = match ev.data().delta() {
                    WheelDelta::Pixels(p) => p.y,
                    WheelDelta::Lines(p)  => p.y * 20.0,
                    WheelDelta::Pages(p)  => p.y * 400.0,
                };
                state.zoom_at(pos.x, pos.y, delta_y);
            },

            // ── touch (single-finger pan + two-finger pinch-zoom) ────────────
            ontouchstart: move |ev| {
                if locked { return; }
                ev.prevent_default();
                let touches = ev.data().touches();
                match touches.len() {
                    1 => {
                        let c0 = touches[0].client_coordinates();
                        state.stop_pinch();
                        state.deselect();
                        state.start_pan(c0.x, c0.y);
                    }
                    2 => {
                        let c0 = touches[0].client_coordinates();
                        let c1 = touches[1].client_coordinates();
                        state.stop_pan();
                        let dist = touch_dist(c0.x, c0.y, c1.x, c1.y);
                        let (ox, oy) = *canvas_origin.read();
                        let cx = (c0.x + c1.x) / 2.0 - ox;
                        let cy = (c0.y + c1.y) / 2.0 - oy;
                        state.start_pinch(dist, cx, cy);
                    }
                    _ => {}
                }
            },
            ontouchmove: move |ev| {
                if locked { return; }
                ev.prevent_default();
                let touches = ev.data().touches();
                match touches.len() {
                    1 if !state.is_pinching() => {
                        let c0 = touches[0].client_coordinates();
                        state.update_pan(c0.x, c0.y);
                    }
                    2 => {
                        let c0 = touches[0].client_coordinates();
                        let c1 = touches[1].client_coordinates();
                        let dist = touch_dist(c0.x, c0.y, c1.x, c1.y);
                        let (ox, oy) = *canvas_origin.read();
                        let cx = (c0.x + c1.x) / 2.0 - ox;
                        let cy = (c0.y + c1.y) / 2.0 - oy;
                        state.update_pinch(dist, cx, cy);
                    }
                    _ => {}
                }
            },
            ontouchend: move |ev| {
                let touches = ev.data().touches();
                match touches.len() {
                    0 => {
                        state.stop_drag();
                        state.stop_pan();
                        state.stop_pinch();
                    }
                    1 => {
                        state.stop_pinch();
                        if !locked {
                            let c0 = touches[0].client_coordinates();
                            state.start_pan(c0.x, c0.y);
                        }
                    }
                    _ => {}
                }
            },
            ontouchcancel: move |_| {
                state.stop_drag();
                state.stop_pan();
                state.stop_pinch();
            },

            // ── world (transformed) ──────────────────────────────────────────
            div {
                style: format!(
                    "position: absolute; top: 0; left: 0; width: 3000px; height: 2000px; transform: {transform}; transform-origin: 0 0;"
                ),

                div {
                    class: "absolute inset-0 pointer-events-none text-foreground",
                    style: "background-image: radial-gradient(circle, currentColor 1px, transparent 1px); background-size: 20px 20px; opacity: 0.12;",
                }

                svg {
                    class: "absolute inset-0 pointer-events-none overflow-visible",
                    width: "3000",
                    height: "2000",
                    style { "@keyframes edge-flow {{ to {{ stroke-dashoffset: -18; }} }}" }
                    defs {
                        marker {
                            id: "wf-arrow",
                            "markerWidth": "8",
                            "markerHeight": "8",
                            "refX": "6",
                            "refY": "3",
                            orient: "auto",
                            "markerUnits": "strokeWidth",
                            path {
                                d: "M0,0 L0,6 L8,3 z",
                                fill: "currentColor",
                                class: "text-border",
                            }
                        }
                    }
                    for (ei, (d, style, lx, ly, label_opt)) in edge_paths.iter().enumerate() {
                        g {
                        path {
                            d: d.as_str(),
                            fill: "none",
                            stroke: "currentColor",
                            class: if selected_ei == Some(ei) { "text-primary" } else { "text-border" },
                            "stroke-width": if selected_ei == Some(ei) { "2.5" } else { "1.5" },
                            "stroke-linecap": "round",
                            "stroke-dasharray": style.dasharray(),
                            "marker-end": "url(#wf-arrow)",
                            style: if style.dasharray() != "none" { "animation: edge-flow 1.2s linear infinite; cursor: pointer; pointer-events: visibleStroke;" } else { "cursor: pointer; pointer-events: visibleStroke;" },
                            oncontextmenu: move |ev| {
                                ev.prevent_default();
                                ev.stop_propagation();
                                let c = ev.data().client_coordinates();
                                edge_cm.set(Some((ei, c.x, c.y)));
                            },
                            onmousedown: move |ev| { ev.stop_propagation(); },
                            onclick: move |ev| {
                                ev.stop_propagation();
                                if state.selected_edge_idx() == Some(ei) {
                                    state.deselect_edge();
                                } else {
                                    state.select_edge(ei);
                                }
                            },
                        }
                        if let Some(label) = label_opt {
                            text {
                                x: "{lx:.1}",
                                y: "{ly:.1}",
                                "text-anchor": "middle",
                                "dominant-baseline": "central",
                                "font-size": "11",
                                "paint-order": "stroke",
                                stroke: "hsl(var(--background))",
                                "stroke-width": "3",
                                fill: "currentColor",
                                class: "text-foreground/80 pointer-events-none select-none",
                                "{label}"
                            }
                        }
                        }
                    }
                    if let Some(preview) = state.connecting_preview() {
                        path {
                            d: preview.as_str(),
                            fill: "none",
                            stroke: "hsl(var(--primary))",
                            "stroke-width": "1.5",
                            "stroke-linecap": "round",
                            "stroke-dasharray": "5 4",
                            style: "opacity: 0.8;",
                        }
                    }
                }

                {children}
            }

            // ── overlays (viewport space, not transformed) ───────────────────
            {overlay}

            if let Some((x, y, w, h)) = rubber_band_rect {
                div {
                    class: "absolute pointer-events-none border border-primary/60 bg-primary/10 rounded-sm",
                    style: "left:{x:.1}px; top:{y:.1}px; width:{w:.1}px; height:{h:.1}px;",
                }
            }

            // ── node context menu overlay ────────────────────────────────────
            if let Some((ni, mx, my)) = *node_cm.read() {
                div {
                    class: "fixed inset-0 z-[49]",
                    onclick: move |_| { node_cm.set(None); },
                    oncontextmenu: move |ev| { ev.prevent_default(); node_cm.set(None); },
                }
                div {
                    class: "fixed z-50 p-1 rounded-md border bg-card shadow-md w-[200px]",
                    style: "left:{mx}px; top:{my}px;",
                    onclick: move |ev| { ev.stop_propagation(); },
                    ContextMenuLabel { "Node" }
                    ContextMenuGroup {
                        button {
                            class: MENU_BTN,
                            onclick: move |_| { state.start_edit(ni); node_cm.set(None); },
                            Pencil { class: "size-3.5 text-muted-foreground" }
                            "Rename"
                        }
                        button {
                            class: MENU_BTN,
                            onclick: move |_| { state.duplicate_node(ni); node_cm.set(None); },
                            Copy { class: "size-3.5 text-muted-foreground" }
                            "Duplicate"
                        }
                    }
                    div { class: "my-1 border-t border-border" }
                    ContextMenuGroup {
                        button {
                            class: MENU_BTN_DANGER,
                            onclick: move |_| {
                                state.select_node(ni);
                                state.delete_selected();
                                node_cm.set(None);
                            },
                            Trash2 { class: "size-3.5" }
                            "Delete"
                        }
                    }
                }
            }

            // ── edge context menu overlay ────────────────────────────────────
            if let Some((ei, mx, my)) = *edge_cm.read() {
                div {
                    class: "fixed inset-0 z-[49]",
                    onclick: move |_| { edge_cm.set(None); },
                    oncontextmenu: move |ev| { ev.prevent_default(); edge_cm.set(None); },
                }
                div {
                    class: "fixed z-50 p-1 rounded-md border bg-card shadow-md w-[200px]",
                    style: "left:{mx}px; top:{my}px;",
                    onclick: move |ev| { ev.stop_propagation(); },
                    ContextMenuLabel { "Edge" }
                    ContextMenuGroup {
                        button {
                            class: MENU_BTN,
                            onclick: move |_| {
                                state.start_edit_edge(ei);
                                edge_edit_vp_pos.set(Some((mx, my)));
                                edge_cm.set(None);
                            },
                            Pencil { class: "size-3.5 text-muted-foreground" }
                            "Edit Label"
                        }
                    }
                    div { class: "my-1 border-t border-border" }
                    ContextMenuGroup {
                        button {
                            class: MENU_BTN_DANGER,
                            onclick: move |_| {
                                state.delete_edge(ei);
                                edge_cm.set(None);
                            },
                            Trash2 { class: "size-3.5" }
                            "Delete Edge"
                        }
                    }
                }
            }
            // ── edge label edit overlay ──────────────────────────────────────
            if state.editing_edge_idx().is_some() {
                if let Some((vp_x, vp_y)) = *edge_edit_vp_pos.read() {
                    div {
                        class: "absolute z-30 flex items-center bg-background border border-primary rounded-md shadow-md px-2 py-1",
                        style: "left:{vp_x:.1}px; top:{vp_y:.1}px; transform: translate(-50%, -50%); min-width: 120px;",
                        onclick: move |ev| { ev.stop_propagation(); },
                        onmousedown: move |ev| { ev.stop_propagation(); },
                        input {
                            class: "w-full bg-transparent outline-none text-xs text-center",
                            placeholder: "Edge label…",
                            value: state.edit_buffer_value(),
                            onmounted: move |ev| {
                                spawn(async move { let _ = ev.set_focus(true).await; });
                            },
                            oninput: move |ev| { state.update_edit_buffer(ev.value()); },
                            onkeydown: move |ev| {
                                match ev.data().key() {
                                    Key::Enter  => { state.commit_edit_edge(); edge_edit_vp_pos.set(None); }
                                    Key::Escape => { state.cancel_edit_edge(); edge_edit_vp_pos.set(None); }
                                    _ => {}
                                }
                            },
                            onblur: move |_| { state.commit_edit_edge(); edge_edit_vp_pos.set(None); },
                        }
                    }
                }
            }

            // ── canvas context menu overlay ──────────────────────────────────
            if let Some((mx, my)) = *canvas_cm.read() {
                div {
                    class: "fixed inset-0 z-[49]",
                    onclick: move |_| { canvas_cm.set(None); },
                    oncontextmenu: move |ev| { ev.prevent_default(); canvas_cm.set(None); },
                }
                div {
                    class: "fixed z-50 p-1 rounded-md border bg-card shadow-md w-[200px]",
                    style: "left:{mx}px; top:{my}px;",
                    onclick: move |ev| { ev.stop_propagation(); },
                    ContextMenuLabel { "Canvas" }
                    ContextMenuGroup {
                        button {
                            class: MENU_BTN,
                            onclick: move |_| {
                                let (ox, oy) = *canvas_origin.read();
                                let (px, py) = *state.pan.read();
                                let zoom = *state.zoom.read();
                                state.add_node((mx - ox - px) / zoom, (my - oy - py) / zoom);
                                canvas_cm.set(None);
                            },
                            Plus { class: "size-3.5 text-muted-foreground" }
                            "Add Node Here"
                        }
                        button {
                            class: MENU_BTN,
                            onclick: move |_| { state.select_all(); canvas_cm.set(None); },
                            SquareDashedMousePointer { class: "size-3.5 text-muted-foreground" }
                            "Select All"
                        }
                        if state.clipboard_count() > 0 {
                            button {
                                class: MENU_BTN,
                                onclick: move |_| { state.paste_nodes(); canvas_cm.set(None); },
                                Clipboard { class: "size-3.5 text-muted-foreground" }
                                "Paste"
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── WorkflowNode ──────────────────────────────────────────────────────────────

#[component]
pub fn WorkflowNodeWrapper(state: WorkflowState, idx: usize, children: Element) -> Element {
    let node = { state.nodes.read().get(idx).cloned() };
    let Some(node) = node else { return rsx! {} };

    let (x, y) = state.pos(idx);
    let is_active = state.active_idx() == Some(idx);
    let is_selected = state.is_selected(idx);
    let is_connecting = state.is_connecting();
    let locked = state.is_locked();
    let is_editing = state.is_editing(idx);
    let edit_value = state.edit_buffer_value();

    let width = node.width;
    let node_id = node.id.clone();
    let from_x = x + width;
    let from_y = y + NODE_H / 2.0;
    let to_id = node.id.clone();
    let mut state = state;
    let mut cm_ctx = use_context::<NodeCmCtx>();

    rsx! {
        div {
            class: format!(
                "absolute transition-shadow{}{}",
                if is_active   { " shadow-lg" } else { "" },
                if is_selected { " ring-2 ring-primary ring-offset-1 ring-offset-background rounded-md" } else { "" },
            ),
            style: format!("left: {x:.1}px; top: {y:.1}px; width: {width:.0}px; cursor: {};", if locked { "default" } else { "grab" }),

            oncontextmenu: move |ev| {
                ev.prevent_default();
                ev.stop_propagation();
                let c = ev.data().client_coordinates();
                cm_ctx.0.set(Some((idx, c.x, c.y)));
            },
            onmousedown: move |ev| {
                ev.stop_propagation();
                if locked { return; }
                if state.is_editing(idx) { return; }
                let data = ev.data();
                let c    = data.client_coordinates();
                let shift = data.modifiers().contains(Modifiers::SHIFT);
                if shift {
                    state.toggle_select(idx);
                } else if !state.is_selected(idx) {
                    state.select_node(idx);
                }
                if !state.is_connecting() && state.is_selected(idx) {
                    state.start_drag(idx, c.x, c.y);
                }
            },
            ondoubleclick: move |ev| {
                ev.stop_propagation();
                if !locked { state.start_edit(idx); }
            },

            {children}

            if is_editing {
                div {
                    class: "absolute inset-0 z-20 flex items-center px-3 bg-background rounded-md ring-2 ring-primary",
                    onclick: move |ev| { ev.stop_propagation(); },
                    onmousedown: move |ev| { ev.stop_propagation(); },
                    input {
                        class: "w-full bg-transparent outline-none text-sm font-medium",
                        value: "{edit_value}",
                        onmounted: move |ev| {
                            spawn(async move { let _ = ev.set_focus(true).await; });
                        },
                        oninput: move |ev| { state.update_edit_buffer(ev.value()); },
                        onkeydown: move |ev| {
                            match ev.data().key() {
                                Key::Enter => { state.finish_edit(); }
                                Key::Escape => { state.cancel_edit(); }
                                _ => {}
                            }
                        },
                        onblur: move |_| { state.finish_edit(); },
                    }
                }
            }

            // ── source handle (right) ─────────────────────────────────────
            if node.has_source {
                div {
                    class: format!(
                        "absolute right-0 top-1/2 size-3 rounded-full border-2 border-primary bg-background transition-colors{}",
                        if is_connecting { " ring-2 ring-primary/40 scale-125" } else { "" },
                    ),
                    style: "transform: translate(50%, -50%); cursor: crosshair; z-index: 10;",
                    "data-testid": "source-handle",
                    "data-node-id": "{node.id}",
                    onmousedown: move |ev| {
                        ev.stop_propagation();
                        if !locked { state.start_connect(node_id.clone(), from_x, from_y); }
                    },
                }
            }

            // ── target handle (left) ──────────────────────────────────────
            if node.has_target {
                div {
                    class: format!(
                        "absolute left-0 top-1/2 size-3 rounded-full border-2 border-primary bg-background transition-colors{}",
                        if is_connecting { " ring-2 ring-primary/40 scale-125" } else { "" },
                    ),
                    style: "transform: translate(-50%, -50%); cursor: crosshair; z-index: 10;",
                    "data-testid": "target-handle",
                    "data-node-id": "{node.id}",
                    onmouseup: move |ev| {
                        ev.stop_propagation();
                        state.finish_connect(to_id.clone());
                    },
                }
            }
        }
    }
}

// ── WorkflowDefaultNode ───────────────────────────────────────────────────────

#[component]
pub fn WorkflowDefaultNode(node: WorkflowNode) -> Element {
    rsx! {
        WfNode {
            target: false,
            source: false,
            WfNodeHeader {
                span { class: format!("size-2 rounded-full shrink-0 {}", node.kind.dot_color()) }
                WfNodeTitle { class: node.kind.text_color(), "{node.label}" }
                span {
                    class: "ml-auto text-[10px] text-muted-foreground uppercase tracking-wide shrink-0",
                    "{node.kind.label()}"
                }
            }
            WfNodeContent {
                WfNodeDescription { class: "font-mono", "{node.description}" }
            }
        }
    }
}

// ── WorkflowMinimap ───────────────────────────────────────────────────────────

#[component]
pub fn WorkflowMinimap(state: WorkflowState) -> Element {
    let scale_x = MINI_W / WORLD_REF_W;
    let scale_y = MINI_H / WORLD_REF_H;
    let mut state = state;

    let pos_snap = state.positions.read().clone();
    let nodes_snap = state.nodes.read().clone();
    let edges_snap = state.edges.read().clone();

    let node_rects: Vec<(f64, f64, f64, f64)> = nodes_snap
        .iter()
        .enumerate()
        .map(|(i, n)| {
            let (x, y) = pos_snap[i];
            (x * scale_x, y * scale_y, n.width * scale_x, NODE_H * scale_y)
        })
        .collect();

    let edge_paths: Vec<String> = edges_snap
        .iter()
        .filter_map(|edge| {
            let (fi, from) = nodes_snap.iter().enumerate().find(|(_, n)| n.id == edge.from)?;
            let (ti, _) = nodes_snap.iter().enumerate().find(|(_, n)| n.id == edge.to)?;
            let (fx, fy) = pos_snap[fi];
            let (tx, ty) = pos_snap[ti];
            let sx = (fx + from.width) * scale_x;
            let sy = (fy + NODE_H / 2.0) * scale_y;
            let tx2 = tx * scale_x;
            let ty2 = (ty + NODE_H / 2.0) * scale_y;
            let dx = (tx2 - sx).abs();
            let off = (dx / 2.0).clamp(4.0, 12.0);
            Some(format!("M {sx:.1} {sy:.1} C {:.1} {sy:.1}, {:.1} {ty2:.1}, {tx2:.1} {ty2:.1}", sx + off, tx2 - off,))
        })
        .collect();

    let (pan_x, pan_y) = *state.pan.read();
    let zoom = state.zoom_value();
    let vp_x = (-pan_x / zoom) * scale_x;
    let vp_y = (-pan_y / zoom) * scale_y;
    let vp_w = (VIEWPORT_W / zoom) * scale_x;
    let vp_h = (VIEWPORT_H / zoom) * scale_y;

    rsx! {
        div {
            class: "rounded-md border bg-background/90 backdrop-blur-sm shadow-sm overflow-hidden cursor-pointer",
            style: format!(
                "position: absolute; bottom: 12px; right: 12px; width: {MINI_W}px; height: {MINI_H}px;"
            ),
            onclick: move |ev| {
                let ec = ev.data().element_coordinates();
                let world_x = ec.x / scale_x;
                let world_y = ec.y / scale_y;
                let zoom = state.zoom_value();
                state.pan.set((
                    VIEWPORT_W / 2.0 - world_x * zoom,
                    VIEWPORT_H / 2.0 - world_y * zoom,
                ));
            },

            svg {
                width: "{MINI_W}",
                height: "{MINI_H}",

                rect {
                    x: "{vp_x:.1}",
                    y: "{vp_y:.1}",
                    width: "{vp_w:.1}",
                    height: "{vp_h:.1}",
                    rx: "2",
                    fill: "hsl(var(--primary) / 0.06)",
                    stroke: "hsl(var(--primary) / 0.5)",
                    "stroke-width": "1",
                    "stroke-dasharray": "3 2",
                }

                for d in &edge_paths {
                    path {
                        d: d.as_str(),
                        fill: "none",
                        stroke: "hsl(var(--border))",
                        "stroke-width": "0.8",
                    }
                }

                for (x, y, w, h) in &node_rects {
                    rect {
                        x: "{x:.1}",
                        y: "{y:.1}",
                        width: "{w:.1}",
                        height: "{h:.1}",
                        rx: "1.5",
                        fill: "hsl(var(--muted-foreground) / 0.3)",
                        stroke: "hsl(var(--muted-foreground) / 0.6)",
                        "stroke-width": "0.5",
                    }
                }
            }
        }
    }
}

// ── WorkflowControls ──────────────────────────────────────────────────────────

#[component]
pub fn WorkflowControls(state: WorkflowState) -> Element {
    let pct = (state.zoom_value() * 100.0).round() as i32;
    let mut state = state;

    rsx! {
        div {
            class: "flex items-center gap-0.5 rounded-md border bg-background/90 backdrop-blur-sm shadow-sm px-1.5 py-1",
            style: "position: absolute; top: 12px; right: 12px;",

            button {
                class: "size-6 flex items-center justify-center rounded text-sm hover:bg-accent transition-colors",
                title: "Add node",
                onclick: move |_| {
                    let (pan_x, pan_y) = *state.pan.read();
                    let zoom = state.zoom_value();
                    let cx = -pan_x / zoom + VIEWPORT_W / 2.0 / zoom;
                    let cy = -pan_y / zoom + VIEWPORT_H / 2.0 / zoom;
                    state.add_node(cx, cy);
                },
                "+"
            }

            div { class: "w-px h-4 bg-border mx-0.5" }

            span {
                class: "text-[11px] text-muted-foreground tabular-nums w-9 text-center",
                "{pct}%"
            }
            button {
                class: "size-6 flex items-center justify-center rounded text-sm hover:bg-accent transition-colors",
                onclick: move |_| { state.zoom_step(1.0 / 1.2); },
                "−"
            }
            button {
                class: "size-6 flex items-center justify-center rounded text-sm hover:bg-accent transition-colors",
                onclick: move |_| { state.zoom_step(1.2); },
                "+"
            }

            div { class: "w-px h-4 bg-border mx-0.5" }

            button {
                class: "text-[11px] px-1.5 py-0.5 rounded hover:bg-accent text-muted-foreground transition-colors",
                onclick: move |_| { state.fit_to_view(VIEWPORT_W, VIEWPORT_H, NODE_H); },
                "fit"
            }
            button {
                class: "text-[11px] px-1.5 py-0.5 rounded hover:bg-accent text-muted-foreground transition-colors",
                onclick: move |_| { state.zoom_reset(); },
                "reset"
            }
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn touch_dist(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    let dx = ax - bx;
    let dy = ay - by;
    (dx * dx + dy * dy).sqrt()
}
