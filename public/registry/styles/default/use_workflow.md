---
title: "Use Workflow"
name: "use_workflow"
cargo_dependencies: ["serde", "strum"]
registry_dependencies: []
type: "components:hooks"
path: "hooks/use_workflow.rs"
description: "This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality."
tags: []
---

# Use Workflow

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_workflow
```

## Component Code

```rust
use std::collections::HashSet;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator as _};

use super::use_history_stack::UseHistoryStack;

// ── WorkflowNodeKind ──────────────────────────────────────────────────────────

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkflowNodeKind {
    Trigger,
    Data,
    Agent,
    Output,
}

impl WorkflowNodeKind {
    pub fn dot_color(&self) -> &'static str {
        match self {
            WorkflowNodeKind::Trigger => "bg-yellow-500",
            WorkflowNodeKind::Data => "bg-blue-500",
            WorkflowNodeKind::Agent => "bg-purple-500",
            WorkflowNodeKind::Output => "bg-green-500",
        }
    }

    pub fn text_color(&self) -> &'static str {
        match self {
            WorkflowNodeKind::Trigger => "text-yellow-600 dark:text-yellow-400",
            WorkflowNodeKind::Data => "text-blue-600 dark:text-blue-400",
            WorkflowNodeKind::Agent => "text-purple-600 dark:text-purple-400",
            WorkflowNodeKind::Output => "text-green-600 dark:text-green-400",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            WorkflowNodeKind::Trigger => "Trigger",
            WorkflowNodeKind::Data => "Data",
            WorkflowNodeKind::Agent => "Agent",
            WorkflowNodeKind::Output => "Output",
        }
    }
}

// ── Data types ────────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq, Default, Display, EnumIter, Serialize, Deserialize)]
pub enum EdgeStyle {
    #[default]
    Dashed,
    Solid,
    Dotted,
}

impl EdgeStyle {
    pub fn dasharray(&self) -> &'static str {
        match self {
            EdgeStyle::Solid => "none",
            EdgeStyle::Dashed => "6 3",
            EdgeStyle::Dotted => "2 3",
        }
    }

    pub fn all() -> impl Iterator<Item = EdgeStyle> {
        EdgeStyle::iter()
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowNode {
    pub id: String,
    pub initial_x: f64,
    pub initial_y: f64,
    pub width: f64,
    pub has_target: bool,
    pub has_source: bool,
    pub label: String,
    pub description: String,
    pub kind: WorkflowNodeKind,
}

#[derive(Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowEdge {
    pub from: String,
    pub to: String,
    pub style: EdgeStyle,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct WorkflowSnapshot {
    pub nodes: Vec<WorkflowNode>,
    pub edges: Vec<WorkflowEdge>,
    pub positions: Vec<(f64, f64)>,
    pub pan: (f64, f64),
    pub zoom: f64,
}

// ── ConnectingState ───────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct ConnectingState {
    pub from_node_id: String,
    pub from_x: f64,
    pub from_y: f64,
    pub mouse_x: f64,
    pub mouse_y: f64,
}

// ── Internal drag/pan state ───────────────────────────────────────────────────

#[derive(Clone)]
pub struct DragState {
    pub node_idx: usize,
    pub mouse_start_x: f64,
    pub mouse_start_y: f64,
    /// Start positions of every node being dragged: (idx, start_x, start_y).
    pub starts: Vec<(usize, f64, f64)>,
}

#[derive(Clone, Copy)]
struct PanState {
    mouse_start_x: f64,
    mouse_start_y: f64,
    pan_start_x: f64,
    pan_start_y: f64,
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct PinchState {
    prev_dist: f64,
    /// Element-relative midpoint between the two fingers.
    cx: f64,
    cy: f64,
}

#[derive(Clone, Copy, PartialEq)]
pub struct RubberBandState {
    pub start_x: f64,
    pub start_y: f64,
    pub cur_x: f64,
    pub cur_y: f64,
}

// ── WorkflowState ─────────────────────────────────────────────────────────────

#[allow(clippy::type_complexity)]
pub struct WorkflowState {
    pub nodes: Signal<Vec<WorkflowNode>>,
    pub edges: Signal<Vec<WorkflowEdge>>,
    pub positions: Signal<Vec<(f64, f64)>>,
    pub drag: Signal<Option<DragState>>,
    pub pan: Signal<(f64, f64)>,
    pub zoom: Signal<f64>,
    pub selected: Signal<HashSet<usize>>,
    pub connecting: Signal<Option<ConnectingState>>,
    canvas_drag: Signal<Option<PanState>>,
    touch_pinch: Signal<Option<PinchState>>,
    pub history: UseHistoryStack<(Vec<(f64, f64)>, Vec<WorkflowEdge>)>,
    next_id: Signal<usize>,
    pub locked: Signal<bool>,
    pub snap_to_grid: Signal<bool>,
    /// Clipboard stores (node, x, y) at time of copy. Each paste offsets by +20.
    clipboard: Signal<Vec<(WorkflowNode, f64, f64)>>,
    pub selected_edge: Signal<Option<usize>>,
    pub editing_node: Signal<Option<usize>>,
    pub editing_edge: Signal<Option<usize>>,
    edit_buffer: Signal<String>,
    pub rubber_band: Signal<Option<RubberBandState>>,
}

// Manual Copy/Clone/PartialEq — Signal<Vec<T>> is Copy regardless of T.
impl Copy for WorkflowState {}
impl Clone for WorkflowState {
    fn clone(&self) -> Self {
        *self
    }
}
impl PartialEq for WorkflowState {
    fn eq(&self, other: &Self) -> bool {
        self.nodes == other.nodes
            && self.edges == other.edges
            && self.positions == other.positions
            && self.drag == other.drag
            && self.pan == other.pan
            && self.zoom == other.zoom
            && self.selected == other.selected
            && self.connecting == other.connecting
            && self.touch_pinch == other.touch_pinch
            && self.history == other.history
            && self.locked == other.locked
            && self.snap_to_grid == other.snap_to_grid
            && self.selected_edge == other.selected_edge
            && self.editing_node == other.editing_node
            && self.editing_edge == other.editing_edge
            && self.rubber_band == other.rubber_band
    }
}

impl WorkflowState {
    // ── selection ────────────────────────────────────────────────────────────

    /// Returns the single selected index (first in set), for backward compat.
    pub fn selected_idx(&self) -> Option<usize> {
        self.selected.read().iter().next().copied()
    }

    pub fn is_selected(&self, idx: usize) -> bool {
        self.selected.read().contains(&idx)
    }

    /// Select only this node (clears multi-selection).
    pub fn select_node(&mut self, idx: usize) {
        self.selected_edge.set(None);
        let mut s = self.selected.write();
        s.clear();
        s.insert(idx);
    }

    /// Add to or remove from current selection (Shift+click).
    pub fn toggle_select(&mut self, idx: usize) {
        self.selected_edge.set(None);
        let mut s = self.selected.write();
        if s.contains(&idx) {
            s.remove(&idx);
        } else {
            s.insert(idx);
        }
    }

    pub fn deselect(&mut self) {
        self.selected.write().clear();
        self.selected_edge.set(None);
    }

    // ── locked mode ───────────────────────────────────────────────────────────

    pub fn is_locked(&self) -> bool {
        *self.locked.read()
    }

    pub fn set_locked(&mut self, v: bool) {
        self.locked.set(v);
    }

    pub fn toggle_locked(&mut self) {
        let v = *self.locked.read();
        self.locked.set(!v);
    }

    pub fn is_snap_to_grid(&self) -> bool {
        *self.snap_to_grid.read()
    }

    pub fn set_snap_to_grid(&mut self, v: bool) {
        self.snap_to_grid.set(v);
    }

    pub fn toggle_snap_to_grid(&mut self) {
        let v = *self.snap_to_grid.read();
        self.snap_to_grid.set(!v);
    }

    // ── connect ───────────────────────────────────────────────────────────────

    pub fn is_connecting(&self) -> bool {
        self.connecting.read().is_some()
    }

    pub fn start_connect(&mut self, from_node_id: String, from_x: f64, from_y: f64) {
        self.connecting.set(Some(ConnectingState { from_node_id, from_x, from_y, mouse_x: from_x, mouse_y: from_y }));
    }

    pub fn update_connect_mouse(&mut self, ex: f64, ey: f64) {
        if self.connecting.read().is_none() {
            return;
        }
        let (pan_x, pan_y) = *self.pan.read();
        let zoom = *self.zoom.read();
        let wx = (ex - pan_x) / zoom;
        let wy = (ey - pan_y) / zoom;
        if let Some(cs) = self.connecting.write().as_mut() {
            cs.mouse_x = wx;
            cs.mouse_y = wy;
        }
    }

    pub fn finish_connect(&mut self, to_node_id: String) {
        let cs = self.connecting.read().clone();
        self.connecting.set(None);
        let Some(cs) = cs else { return };
        if cs.from_node_id == to_node_id {
            return;
        }
        let already = self.edges.read().iter().any(|e| e.from == cs.from_node_id && e.to == to_node_id);
        if !already {
            self.edges.write().push(WorkflowEdge { from: cs.from_node_id, to: to_node_id, ..Default::default() });
            self.push_history();
        }
    }

    // ── edge selection ────────────────────────────────────────────────────────

    pub fn select_edge(&mut self, idx: usize) {
        self.selected.write().clear();
        self.selected_edge.set(Some(idx));
    }

    pub fn deselect_edge(&mut self) {
        self.selected_edge.set(None);
    }

    pub fn selected_edge_idx(&self) -> Option<usize> {
        *self.selected_edge.read()
    }

    pub fn delete_selected_edge(&mut self) {
        let Some(idx) = *self.selected_edge.read() else { return };
        if idx < self.edges.read().len() {
            self.edges.write().remove(idx);
        }
        self.selected_edge.set(None);
        self.push_history();
    }

    // ── inline edit ───────────────────────────────────────────────────────────

    pub fn is_editing(&self, idx: usize) -> bool {
        *self.editing_node.read() == Some(idx)
    }

    pub fn start_edit(&mut self, idx: usize) {
        let label = self.nodes.read()[idx].label.clone();
        self.edit_buffer.set(label);
        self.editing_node.set(Some(idx));
    }

    pub fn update_edit_buffer(&mut self, val: String) {
        self.edit_buffer.set(val);
    }

    pub fn finish_edit(&mut self) {
        let Some(idx) = *self.editing_node.read() else { return };
        let val = self.edit_buffer.read().clone();
        if let Some(node) = self.nodes.write().get_mut(idx) {
            node.label = val;
        }
        self.editing_node.set(None);
    }

    pub fn cancel_edit(&mut self) {
        self.editing_node.set(None);
    }

    // ── edge label edit ───────────────────────────────────────────────────────

    pub fn editing_edge_idx(&self) -> Option<usize> {
        *self.editing_edge.read()
    }

    pub fn start_edit_edge(&mut self, idx: usize) {
        let label = self.edges.read().get(idx).and_then(|e| e.label.clone()).unwrap_or_default();
        self.edit_buffer.set(label);
        self.editing_node.set(None);
        self.editing_edge.set(Some(idx));
    }

    pub fn commit_edit_edge(&mut self) {
        let Some(idx) = *self.editing_edge.read() else { return };
        let val = self.edit_buffer.read().clone();
        if let Some(edge) = self.edges.write().get_mut(idx) {
            edge.label = if val.trim().is_empty() { None } else { Some(val) };
        }
        self.editing_edge.set(None);
        self.push_history();
    }

    pub fn cancel_edit_edge(&mut self) {
        self.editing_edge.set(None);
    }

    pub fn edit_buffer_value(&self) -> String {
        self.edit_buffer.read().clone()
    }

    pub fn cancel_connect(&mut self) {
        self.connecting.set(None);
    }

    // ── rubber-band select ────────────────────────────────────────────────────

    pub fn start_rubber_band(&mut self, x: f64, y: f64) {
        self.rubber_band.set(Some(RubberBandState { start_x: x, start_y: y, cur_x: x, cur_y: y }));
    }

    pub fn update_rubber_band(&mut self, x: f64, y: f64) {
        if let Some(rb) = self.rubber_band.write().as_mut() {
            rb.cur_x = x;
            rb.cur_y = y;
        }
    }

    /// Commit: select all nodes whose AABB intersects the rubber-band rect.
    /// `canvas_origin` is the element's top-left in client coords (from `onmounted`).
    pub fn finish_rubber_band(&mut self, node_h: f64, canvas_origin: (f64, f64)) {
        let Some(rb) = *self.rubber_band.read() else { return };
        self.rubber_band.set(None);
        let (ox, oy) = canvas_origin;
        let rx0 = rb.start_x.min(rb.cur_x) - ox;
        let ry0 = rb.start_y.min(rb.cur_y) - oy;
        let rx1 = rb.start_x.max(rb.cur_x) - ox;
        let ry1 = rb.start_y.max(rb.cur_y) - oy;
        let (px, py) = *self.pan.read();
        let zoom = *self.zoom.read();
        let wx0 = (rx0 - px) / zoom;
        let wy0 = (ry0 - py) / zoom;
        let wx1 = (rx1 - px) / zoom;
        let wy1 = (ry1 - py) / zoom;
        let nodes = self.nodes.read();
        let positions = self.positions.read();
        let mut sel = self.selected.write();
        for (i, node) in nodes.iter().enumerate() {
            let (nx, ny) = positions[i];
            if nx < wx1 && nx + node.width > wx0 && ny < wy1 && ny + node_h > wy0 {
                sel.insert(i);
            }
        }
    }

    pub fn cancel_rubber_band(&mut self) {
        self.rubber_band.set(None);
    }

    // ── export / import ───────────────────────────────────────────────────────

    pub fn export_snapshot(&self) -> WorkflowSnapshot {
        WorkflowSnapshot {
            nodes: self.nodes.read().clone(),
            edges: self.edges.read().clone(),
            positions: self.positions.read().clone(),
            pan: *self.pan.read(),
            zoom: *self.zoom.read(),
        }
    }

    pub fn load_snapshot(&mut self, snap: WorkflowSnapshot) {
        let next_id = snap.nodes.len();
        // positions BEFORE nodes: render loop indexes positions[idx] immediately
        // after nodes write triggers re-render — wrong order = index-out-of-bounds
        self.positions.set(snap.positions);
        self.nodes.set(snap.nodes);
        self.edges.set(snap.edges);
        self.pan.set(snap.pan);
        self.zoom.set(snap.zoom);
        self.next_id.set(next_id);
        self.selected.write().clear();
        self.drag.set(None);
        self.connecting.set(None);
    }

    pub fn connecting_preview(&self) -> Option<String> {
        let cs = self.connecting.read().clone()?;
        Some(bezier_path(cs.from_x, cs.from_y, cs.mouse_x, cs.mouse_y))
    }

    // ── history ───────────────────────────────────────────────────────────────

    fn push_history(&mut self) {
        let snap = (self.positions.read().clone(), self.edges.read().clone());
        self.history.push(snap);
    }

    // ── delete ───────────────────────────────────────────────────────────────

    pub fn delete_selected(&mut self) {
        let mut indices: Vec<usize> = self.selected.read().iter().cloned().collect();
        if indices.is_empty() {
            return;
        }
        indices.sort_unstable_by(|a, b| b.cmp(a));

        let ids: Vec<String> = {
            let nodes = self.nodes.read();
            indices.iter().filter_map(|&i| nodes.get(i).map(|n| n.id.clone())).collect()
        };

        for &idx in &indices {
            self.nodes.write().remove(idx);
            self.positions.write().remove(idx);
        }
        for id in &ids {
            self.edges.write().retain(|e| e.from != *id && e.to != *id);
        }

        self.selected.write().clear();
        self.push_history();
    }

    // ── keyboard nudge ───────────────────────────────────────────────────────

    pub fn nudge_selected(&mut self, dx: f64, dy: f64) {
        let indices: Vec<usize> = self.selected.read().iter().cloned().collect();
        if indices.is_empty() {
            return;
        }
        let snap = *self.snap_to_grid.read();
        {
            let mut pos = self.positions.write();
            for i in indices {
                let (x, y) = pos[i];
                let raw_x = (x + dx).max(0.0);
                let raw_y = (y + dy).max(0.0);
                pos[i] =
                    if snap { ((raw_x / 20.0).round() * 20.0, (raw_y / 20.0).round() * 20.0) } else { (raw_x, raw_y) };
            }
        }
        self.push_history();
    }

    // ── copy / paste ─────────────────────────────────────────────────────────

    pub fn copy_selected(&mut self) {
        let sel: Vec<usize> = self.selected.read().iter().cloned().collect();
        if sel.is_empty() {
            return;
        }
        let nodes = self.nodes.read();
        let pos = self.positions.read();
        let cb: Vec<(WorkflowNode, f64, f64)> =
            sel.iter().filter_map(|&i| nodes.get(i).map(|n| (n.clone(), pos[i].0, pos[i].1))).collect();
        self.clipboard.set(cb);
    }

    pub fn paste_nodes(&mut self) {
        let clipboard = self.clipboard.read().clone();
        if clipboard.is_empty() {
            return;
        }

        let mut new_indices = Vec::new();
        let mut new_clipboard = Vec::new();

        for (node, x, y) in &clipboard {
            let nx = x + 20.0;
            let ny = y + 20.0;
            new_clipboard.push((node.clone(), nx, ny));

            let n = *self.next_id.read();
            *self.next_id.write() = n + 1;
            let new_node = WorkflowNode { id: format!("node-{n}"), initial_x: nx, initial_y: ny, ..node.clone() };
            let new_idx = self.positions.read().len();
            self.positions.write().push((nx, ny));
            self.nodes.write().push(new_node);
            new_indices.push(new_idx);
        }

        self.clipboard.set(new_clipboard);

        {
            let mut sel = self.selected.write();
            sel.clear();
            for idx in new_indices {
                sel.insert(idx);
            }
        }

        self.push_history();
    }

    pub fn has_clipboard(&self) -> bool {
        !self.clipboard.read().is_empty()
    }

    pub fn clipboard_count(&self) -> usize {
        self.clipboard.read().len()
    }

    pub fn duplicate_selected(&mut self) {
        let sel: Vec<usize> = self.selected.read().iter().cloned().collect();
        if sel.is_empty() {
            return;
        }
        let mut new_indices = Vec::new();
        for idx in &sel {
            let (node, x, y) = {
                let nodes = self.nodes.read();
                let pos = self.positions.read();
                let Some(node) = nodes.get(*idx) else { continue };
                (node.clone(), pos[*idx].0, pos[*idx].1)
            };
            let nx = x + 20.0;
            let ny = y + 20.0;
            let n = *self.next_id.read();
            *self.next_id.write() = n + 1;
            let new_node = WorkflowNode { id: format!("node-{n}"), initial_x: nx, initial_y: ny, ..node };
            let new_idx = self.positions.read().len();
            self.positions.write().push((nx, ny));
            self.nodes.write().push(new_node);
            new_indices.push(new_idx);
        }
        {
            let mut sel = self.selected.write();
            sel.clear();
            for idx in new_indices {
                sel.insert(idx);
            }
        }
        self.push_history();
    }

    // ── context menu helpers ─────────────────────────────────────────────────

    pub fn duplicate_node(&mut self, idx: usize) {
        let (node, x, y) = {
            let nodes = self.nodes.read();
            let pos = self.positions.read();
            let Some(node) = nodes.get(idx) else { return };
            (node.clone(), pos[idx].0, pos[idx].1)
        };
        let nx = x + 20.0;
        let ny = y + 20.0;
        let n = *self.next_id.read();
        *self.next_id.write() = n + 1;
        let new_node = WorkflowNode { id: format!("node-{n}"), initial_x: nx, initial_y: ny, ..node };
        let new_idx = self.positions.read().len();
        self.positions.write().push((nx, ny));
        self.nodes.write().push(new_node);
        {
            let mut sel = self.selected.write();
            sel.clear();
            sel.insert(new_idx);
        }
        self.push_history();
    }

    pub fn delete_edge(&mut self, idx: usize) {
        if idx < self.edges.read().len() {
            self.edges.write().remove(idx);
            if self.selected_edge_idx() == Some(idx) {
                self.selected_edge.set(None);
            }
            self.push_history();
        }
    }

    pub fn select_all(&mut self) {
        let n = self.nodes.read().len();
        let mut sel = self.selected.write();
        sel.clear();
        for i in 0..n {
            sel.insert(i);
        }
    }

    // ── add node ─────────────────────────────────────────────────────────────

    pub fn add_node(&mut self, x: f64, y: f64) {
        let n = *self.next_id.read();
        *self.next_id.write() = n + 1;
        let snap = *self.snap_to_grid.read();
        let snap_x = if snap { (x / 20.0).round() * 20.0 } else { x };
        let snap_y = if snap { (y / 20.0).round() * 20.0 } else { y };
        // positions must be pushed before nodes: writing nodes triggers a re-render,
        // and the render loop indexes into positions by node idx — if positions is
        // shorter than nodes at that moment, pos() panics with index out of bounds.
        self.positions.write().push((snap_x, snap_y));
        self.nodes.write().push(WorkflowNode {
            id: format!("node-{n}"),
            initial_x: snap_x,
            initial_y: snap_y,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: format!("Node {n}"),
            description: "New node".to_string(),
            kind: WorkflowNodeKind::Agent,
        });
    }

    // ── node drag ────────────────────────────────────────────────────────────

    pub fn pos(&self, idx: usize) -> (f64, f64) {
        self.positions.read()[idx]
    }

    pub fn is_dragging(&self) -> bool {
        self.drag.read().is_some()
    }

    pub fn active_idx(&self) -> Option<usize> {
        self.drag.read().as_ref().map(|d| d.node_idx)
    }

    pub fn start_drag(&mut self, idx: usize, mx: f64, my: f64) {
        if !self.selected.read().contains(&idx) {
            self.select_node(idx);
        }
        let starts: Vec<(usize, f64, f64)> = {
            let pos = self.positions.read();
            let sel = self.selected.read();
            sel.iter().map(|&i| (i, pos[i].0, pos[i].1)).collect()
        };
        self.drag.set(Some(DragState { node_idx: idx, mouse_start_x: mx, mouse_start_y: my, starts }));
    }

    pub fn update_drag(&mut self, mx: f64, my: f64) {
        let d = match self.drag.read().clone() {
            Some(d) => d,
            None => return,
        };
        let z = *self.zoom.read();
        let snap = *self.snap_to_grid.read();
        let dx = (mx - d.mouse_start_x) / z;
        let dy = (my - d.mouse_start_y) / z;
        for (idx, sx, sy) in &d.starts {
            let raw_x = (sx + dx).max(0.0);
            let raw_y = (sy + dy).max(0.0);
            self.positions.write()[*idx] =
                if snap { ((raw_x / 20.0).round() * 20.0, (raw_y / 20.0).round() * 20.0) } else { (raw_x, raw_y) };
        }
    }

    pub fn stop_drag(&mut self) {
        if self.drag.read().is_some() {
            self.push_history();
        }
        self.drag.set(None);
    }

    pub fn undo(&mut self) {
        if let Some((pos, edges)) = self.history.undo() {
            *self.positions.write() = pos;
            *self.edges.write() = edges;
        }
    }

    pub fn redo(&mut self) {
        if let Some((pos, edges)) = self.history.redo() {
            *self.positions.write() = pos;
            *self.edges.write() = edges;
        }
    }

    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }
    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }

    // ── canvas pan ───────────────────────────────────────────────────────────

    pub fn is_panning(&self) -> bool {
        self.canvas_drag.read().is_some()
    }

    pub fn start_pan(&mut self, mx: f64, my: f64) {
        let (px, py) = *self.pan.read();
        self.canvas_drag.set(Some(PanState { mouse_start_x: mx, mouse_start_y: my, pan_start_x: px, pan_start_y: py }));
    }

    pub fn update_pan(&mut self, mx: f64, my: f64) {
        if let Some(d) = *self.canvas_drag.read() {
            self.pan.set((d.pan_start_x + mx - d.mouse_start_x, d.pan_start_y + my - d.mouse_start_y));
        }
    }

    pub fn stop_pan(&mut self) {
        self.canvas_drag.set(None);
    }

    // ── touch pinch ───────────────────────────────────────────────────────────

    pub fn is_pinching(&self) -> bool {
        self.touch_pinch.read().is_some()
    }

    pub fn start_pinch(&mut self, dist: f64, cx: f64, cy: f64) {
        self.stop_pan();
        self.touch_pinch.set(Some(PinchState { prev_dist: dist, cx, cy }));
    }

    pub fn update_pinch(&mut self, dist: f64, cx: f64, cy: f64) {
        let prev = match *self.touch_pinch.read() {
            Some(p) => p,
            None => return,
        };
        if prev.prev_dist > 1.0 {
            let scale = dist / prev.prev_dist;
            self.zoom_at_scale(cx, cy, scale);
        }
        self.touch_pinch.set(Some(PinchState { prev_dist: dist, cx, cy }));
    }

    pub fn stop_pinch(&mut self) {
        self.touch_pinch.set(None);
    }

    /// Zoom around (ex, ey) by an arbitrary scale factor — used by pinch gesture.
    pub fn zoom_at_scale(&mut self, ex: f64, ey: f64, scale: f64) {
        let old_z = *self.zoom.read();
        let new_z = (old_z * scale).clamp(0.2, 4.0);
        let ratio = new_z / old_z;
        let (px, py) = *self.pan.read();
        self.pan.set((ex - (ex - px) * ratio, ey - (ey - py) * ratio));
        self.zoom.set(new_z);
    }

    // ── zoom ─────────────────────────────────────────────────────────────────

    pub fn zoom_value(&self) -> f64 {
        *self.zoom.read()
    }

    pub fn zoom_at(&mut self, ex: f64, ey: f64, delta_y: f64) {
        let old_z = *self.zoom.read();
        let factor = if delta_y < 0.0 { 1.1_f64 } else { 1.0 / 1.1 };
        let new_z = (old_z * factor).clamp(0.2, 4.0);
        let ratio = new_z / old_z;
        let (px, py) = *self.pan.read();
        self.pan.set((ex - (ex - px) * ratio, ey - (ey - py) * ratio));
        self.zoom.set(new_z);
    }

    pub fn zoom_step(&mut self, factor: f64) {
        let old_z = *self.zoom.read();
        let new_z = (old_z * factor).clamp(0.2, 4.0);
        let ratio = new_z / old_z;
        let cx = 400.0_f64;
        let cy = 225.0_f64;
        let (px, py) = *self.pan.read();
        self.pan.set((cx - (cx - px) * ratio, cy - (cy - py) * ratio));
        self.zoom.set(new_z);
    }

    pub fn zoom_reset(&mut self) {
        self.pan.set((0.0, 0.0));
        self.zoom.set(1.0);
    }

    pub fn world_transform(&self) -> String {
        let (px, py) = *self.pan.read();
        let z = *self.zoom.read();
        format!("translate({px:.2}px, {py:.2}px) scale({z:.4})")
    }

    // ── fit to view ───────────────────────────────────────────────────────────

    pub fn fit_to_view(&mut self, viewport_w: f64, viewport_h: f64, node_h: f64) {
        let nodes = self.nodes.read();
        if nodes.is_empty() {
            return;
        }
        let padding = 48.0;
        let pos = self.positions.read();
        let min_x = nodes.iter().enumerate().map(|(i, _)| pos[i].0).fold(f64::INFINITY, f64::min);
        let min_y = nodes.iter().enumerate().map(|(i, _)| pos[i].1).fold(f64::INFINITY, f64::min);
        let max_x = nodes.iter().enumerate().map(|(i, n)| pos[i].0 + n.width).fold(f64::NEG_INFINITY, f64::max);
        let max_y = nodes.iter().enumerate().map(|(i, _)| pos[i].1 + node_h).fold(f64::NEG_INFINITY, f64::max);
        drop(pos);
        drop(nodes);
        let content_w = (max_x - min_x).max(1.0);
        let content_h = (max_y - min_y).max(1.0);
        let z =
            ((viewport_w - padding * 2.0) / content_w).min((viewport_h - padding * 2.0) / content_h).clamp(0.2, 4.0);
        self.zoom.set(z);
        self.pan.set(((viewport_w - content_w * z) / 2.0 - min_x * z, (viewport_h - content_h * z) / 2.0 - min_y * z));
    }

    // ── edges ────────────────────────────────────────────────────────────────

    pub fn edge_paths(&self, node_h: f64) -> Vec<(String, EdgeStyle, f64, f64, Option<String>)> {
        let pos = self.positions.read();
        let nodes = self.nodes.read();
        let edges = self.edges.read();
        edges
            .iter()
            .filter_map(|edge| {
                let (fi, from) = nodes.iter().enumerate().find(|(_, n)| n.id == edge.from)?;
                let (ti, _) = nodes.iter().enumerate().find(|(_, n)| n.id == edge.to)?;
                let (fx, fy) = pos[fi];
                let (tx, ty) = pos[ti];
                let sx = fx + from.width;
                let sy = fy + node_h / 2.0;
                let tx2 = tx;
                let ty2 = ty + node_h / 2.0;
                Some((
                    bezier_path(sx, sy, tx2, ty2),
                    edge.style.clone(),
                    (sx + tx2) / 2.0,
                    (sy + ty2) / 2.0,
                    edge.label.clone(),
                ))
            })
            .collect()
    }

    pub fn set_edge_style(&mut self, idx: usize, style: EdgeStyle) {
        if let Some(edge) = self.edges.write().get_mut(idx) {
            edge.style = style;
        }
        self.push_history();
    }
}

// ── Constructor ───────────────────────────────────────────────────────────────

pub fn use_workflow(nodes: Vec<WorkflowNode>, edges: Vec<WorkflowEdge>) -> WorkflowState {
    let initial: Vec<(f64, f64)> = nodes.iter().map(|n| (n.initial_x, n.initial_y)).collect();
    let next_id = nodes.len();
    let initial_edges = edges.clone();
    WorkflowState {
        nodes: use_signal(|| nodes),
        edges: use_signal(|| edges),
        positions: use_signal(|| initial.clone()),
        drag: use_signal(|| None),
        pan: use_signal(|| (0.0, 0.0)),
        zoom: use_signal(|| 1.0),
        selected: use_signal(HashSet::new),
        connecting: use_signal(|| None),
        canvas_drag: use_signal(|| None),
        touch_pinch: use_signal(|| None),
        history: UseHistoryStack::new((initial, initial_edges)),
        next_id: use_signal(|| next_id),
        locked: use_signal(|| false),
        snap_to_grid: use_signal(|| false),
        clipboard: use_signal(Vec::new),
        selected_edge: use_signal(|| None),
        editing_node: use_signal(|| None),
        editing_edge: use_signal(|| None),
        edit_buffer: use_signal(String::new),
        rubber_band: use_signal(|| None),
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn bezier_path(sx: f64, sy: f64, tx: f64, ty: f64) -> String {
    let dx = (tx - sx).abs();
    let offset = (dx / 2.0).clamp(40.0, 80.0);
    format!("M {sx:.1} {sy:.1} C {:.1} {sy:.1}, {:.1} {ty:.1}, {tx:.1} {ty:.1}", sx + offset, tx - offset,)
}
```
