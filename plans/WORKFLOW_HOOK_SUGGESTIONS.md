# Workflow Hook Suggestions

Target file: `src/domain/test/hooks/use_workflow.rs`

Reference idea source: the TypeScript workflow store shared in chat.

## High-value improvements

### 1. Add workflow-level invariants for connections

The TS store centralizes connection checks before mutating edges:

- reject self-links
- reject duplicate edges
- reject links to missing nodes
- reject links from nodes without `has_source`
- reject links into nodes without `has_target`

In the Rust hook, `finish_connect()` currently handles only:

- no-op if source == target
- no-op if the exact edge already exists

Suggested addition:

- add a pure helper like `can_connect(from_id: &str, to_id: &str) -> bool`
- optionally add `normalize_edges(nodes, edges) -> Vec<WorkflowEdge>` for import/init/undo safety

This keeps all structural validation in one place instead of spreading checks across UI handlers.

### 2. Normalize initial and restored edge state

The TS store validates after initialization and structural changes. The Rust hook currently accepts initial edges as-is in `use_workflow(...)`.

That means stale edges can survive if:

- a demo provides an invalid edge
- future import/export restores outdated node ids
- undo/redo restores edges that no longer match the node set

Suggested addition:

- normalize `edges` inside `use_workflow(...)`
- normalize again after snapshot restore if history/import is added later

### 3. History should capture structural state, not just positions/edges

Current history type:

```rust
UseHistoryStack<(Vec<(f64, f64)>, Vec<WorkflowEdge>)>
```

This means undo/redo does not fully restore:

- node additions
- node deletions
- pasted nodes
- label edits

Suggested direction:

```rust
type WorkflowSnapshot = (Vec<WorkflowNode>, Vec<(f64, f64)>, Vec<WorkflowEdge>);
```

Then:

- `push_history()` stores nodes + positions + edges
- `undo()` / `redo()` restore all three

This is the largest functional gap in the current hook.

### 4. Keep transient selection/edit state coherent after mutations

The TS store updates selection-related state in tandem with structural changes. In Rust, after delete/undo/redo, transient indices can become stale:

- `selected`
- `selected_edge`
- `editing_node`
- `drag`

Suggested addition:

- add a small `sync_transient_state()` helper
- clear or clamp indices after delete/undo/redo/import

Without this, index-based UI state can point at removed nodes or edges.

## Medium-value improvements

### 5. Separate structural mutations from visual mutations

The TS store only re-validates on meaningful structural changes. Similar separation would help here:

- structural: add/delete node, connect/disconnect edge, paste/import
- visual: drag, pan, zoom, selection

Suggested direction:

- create small internal helpers such as `mutate_structure(...)` and `mutate_view(...)`
- only snapshot history on structural changes or drag completion

That reduces accidental inconsistency and makes future validation cheaper.

### 6. Add ID-based lookup helpers

The TS store has `getNodeById()` and always treats node ids as the stable key. The Rust hook still mixes:

- stable ids for edges
- transient indices for selection/editing

Suggested helpers:

```rust
fn node_idx_by_id(&self, id: &str) -> Option<usize>
fn node_by_id(&self, id: &str) -> Option<WorkflowNode>
```

This will simplify connection validation, import/export, and future toolbar actions.

### 7. Make add/edit operations history-aware

Current behavior is uneven:

- `paste_nodes()` pushes history
- `delete_selected()` pushes history
- `add_node()` does not
- `finish_edit()` does not

Suggested rule:

- if the action changes user-authored workflow data, it should usually be undoable

At minimum:

- `add_node()` should snapshot history
- decide explicitly whether inline label edits should be undoable

## Optional future work

### 8. Add a lightweight validation state

The TS store keeps:

- `valid`
- `errors`
- `warnings`
- `lastValidated`

Rust does not need the whole system immediately, but a minimal version could still help demos:

- `Signal<Vec<String>>` or typed error enum
- recompute only on structural changes
- surface invalid edges/nodes in controls or badges

### 9. Consider a small import/export snapshot type

There is already an `EXPORT_IMPORT_PLAN.md` nearby. If that work happens, it should share the same normalized snapshot model used by history:

- `nodes`
- `positions`
- `edges`

Using one canonical snapshot shape avoids duplicated state-restoration logic.

## Practical implementation order

1. Add `can_connect(...)` and `normalize_edges(...)`
2. Normalize initial edges in `use_workflow(...)`
3. Upgrade history to store `nodes + positions + edges`
4. Add `sync_transient_state()` after delete/undo/redo
5. Decide whether inline edits belong in undo history

## Bottom line

If only one thing gets fixed, make history structural.  
If a second thing gets fixed, centralize connection validation.
