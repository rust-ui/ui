# Rubber-Band Multi-Select — Implementation Plan

## Behavior

Hold **Shift** and drag on empty canvas → draws a selection rectangle → on mouseup, all nodes whose bounding box intersects the rect are added to `selected`.

Without Shift: canvas drag = pan (unchanged).  
With Shift: canvas drag = rubber-band (no pan).

Shift+click on node still works as before (`toggle_select`).

---

## Conflict Resolution

Current `onmousedown` on canvas background (workflow.rs:92-102):
```rust
onmousedown: move |ev| {
    state.deselect();          // ← must NOT fire when Shift held
    state.start_pan(c.x, c.y);  // ← must NOT fire when Shift held
}
```

New logic:
```rust
onmousedown: move |ev| {
    let shift = ev.data().modifiers().shift();
    if shift {
        state.start_rubber_band(c.x, c.y);   // new
    } else {
        state.finish_edit();
        state.deselect();
        state.start_pan(c.x, c.y);
    }
}
```

---

## State Shape

### New struct in `use_workflow.rs`

```rust
#[derive(Clone, Copy)]
pub struct RubberBandState {
    pub start_x: f64,   // viewport coords (clientX at mousedown)
    pub start_y: f64,
    pub cur_x: f64,     // viewport coords (current mouse position)
    pub cur_y: f64,
}
```

### Added to `WorkflowState`

```rust
pub rubber_band: Signal<Option<RubberBandState>>,
```

### New methods

```rust
pub fn start_rubber_band(&mut self, x: f64, y: f64) {
    self.rubber_band.set(Some(RubberBandState { start_x: x, start_y: y, cur_x: x, cur_y: y }));
}

pub fn update_rubber_band(&mut self, x: f64, y: f64) {
    if let Some(rb) = self.rubber_band.write().as_mut() {
        rb.cur_x = x;
        rb.cur_y = y;
    }
}

pub fn finish_rubber_band(&mut self) {
    let Some(rb) = self.rubber_band.read().clone() else { return; };
    self.rubber_band.set(None);

    // rect in viewport coords
    let rx0 = rb.start_x.min(rb.cur_x);
    let ry0 = rb.start_y.min(rb.cur_y);
    let rx1 = rb.start_x.max(rb.cur_x);
    let ry1 = rb.start_y.max(rb.cur_y);

    // convert to world coords
    let (px, py) = *self.pan.read();
    let zoom = *self.zoom.read();

    let wx0 = (rx0 - px) / zoom;
    let wy0 = (ry0 - py) / zoom;
    let wx1 = (rx1 - px) / zoom;
    let wy1 = (ry1 - py) / zoom;

    let nodes = self.nodes.read();
    let positions = self.positions.read();
    let node_h = 64.0; // same constant used in edge routing

    let mut sel = self.selected.write();
    for (i, node) in nodes.iter().enumerate() {
        let (nx, ny) = positions[i];
        // node AABB
        let nx1 = nx + node.width;
        let ny1 = ny + node_h;
        // AABB intersection
        if nx < wx1 && nx1 > wx0 && ny < wy1 && ny1 > wy0 {
            sel.insert(i);
        }
    }
}

pub fn cancel_rubber_band(&mut self) {
    self.rubber_band.set(None);
}
```

---

## Coordinate System

Viewport origin (clientX/Y) → world coords:
```
world_x = (viewport_x - pan_x) / zoom
world_y = (viewport_y - pan_y) / zoom
```

`positions[i]` is already in world coords. Node AABB = `(x, y, x + width, y + node_h)`.

The canvas `onmousemove` event gives `client_coordinates()` — same space as `clientX/Y`.  
However `canvas_origin` (the element offset) must be subtracted if using `element_coordinates`.  
Use `client_coordinates()` throughout to stay consistent with pan state.

---

## Render: Selection Rectangle

Render inside `WorkflowCanvas` **outside** the transform layer (fixed overlay, like context menu). Uses viewport coords directly — no transform needed.

```
WorkflowCanvas
 ├── transform div (pan/zoom)
 │    ├── edges SVG
 │    └── nodes
 └── overlay
      ├── toolbar / controls
      ├── RubberBandRect  ← new, absolute positioned
      └── ContextMenuOverlay
```

```rust
// In WorkflowCanvas overlay area
if let Some(rb) = state.rubber_band.read().clone() {
    let x = rb.start_x.min(rb.cur_x);
    let y = rb.start_y.min(rb.cur_y);
    let w = (rb.cur_x - rb.start_x).abs();
    let h = (rb.cur_y - rb.start_y).abs();
    div {
        class: "absolute pointer-events-none border border-primary/60 bg-primary/10",
        style: "left:{x}px; top:{y}px; width:{w}px; height:{h}px;",
    }
}
```

Note: `absolute` works here because the canvas root is `relative`. The viewport coords from `clientX/Y` must be offset by the canvas element's bounding rect. Use `canvas_origin` signal (already tracked in the component) to subtract the element offset.

Corrected:
```rust
let (ox, oy) = *canvas_origin.read();
let x = rb.start_x.min(rb.cur_x) - ox;
let y = rb.start_y.min(rb.cur_y) - oy;
```

---

## Wire-up in `workflow.rs`

```
onmousedown  → if shift: start_rubber_band   else: deselect + start_pan
onmousemove  → update_rubber_band (if active) + existing update_drag/update_pan
onmouseup    → finish_rubber_band + stop_drag + stop_pan
onmouseleave → cancel_rubber_band + stop_drag + stop_pan
```

`finish_rubber_band` adds to selection (union, not replace) — Shift-drag accumulates.

---

## Visual Appearance

```
┌──────────────────────────────────────────────────────┐
│                                                      │
│   ┌────────┐  ╔══════════════════╗  ┌────────┐      │
│   │ Input  │  ║  (selection rect)║  │ Output │      │
│   └────────┘  ║  ┌──────────┐   ║  └────────┘      │
│               ║  │Processor │   ║                   │
│               ║  └──────────┘   ║                   │
│               ╚══════════════════╝                   │
│    Shift + drag → dashed blue rect                   │
│    Intersected nodes highlight on release            │
└──────────────────────────────────────────────────────┘
```

---

## Files to Change

| File | Change |
|------|--------|
| `hooks/use_workflow.rs` | `RubberBandState`, field, 4 new methods |
| `components/workflow.rs` | onmousedown Shift branch, onmouseup/leave cleanup, `RubberBandRect` render |
| `demos/demo_workflow_multiselect.rs` | update hint text to mention Shift+drag |

No new demo needed — extend existing `demo_workflow_multiselect`.

---

## Status

- [ ] `RubberBandState` in `use_workflow.rs`
- [ ] `rubber_band` field on `WorkflowState`
- [ ] `start_rubber_band` method
- [ ] `update_rubber_band` method
- [ ] `finish_rubber_band` method (AABB intersection → selection)
- [ ] `cancel_rubber_band` method
- [ ] `onmousedown` Shift branch in `workflow.rs`
- [ ] `onmousemove` calls `update_rubber_band`
- [ ] `onmouseup` / `onmouseleave` call `finish/cancel_rubber_band`
- [ ] `RubberBandRect` overlay element
- [ ] `canvas_origin` offset applied to rect position
- [ ] Demo hint text updated
- [ ] `cargo check` passes
