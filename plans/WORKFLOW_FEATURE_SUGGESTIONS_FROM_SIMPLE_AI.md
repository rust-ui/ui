# Workflow Feature Suggestions From Simple AI

Target area: `src/domain/test`

Scope of this note: feature ideas only.  
Not implementation notes, not architecture notes.

Reference inspiration: `simple-ai.dev/ai-workflows`

## What stands out at the feature level

The product direction is not just "draw boxes and edges". It feels like a workflow builder with:

- a real node catalog
- typed inputs and outputs
- inline configuration panels
- execution visibility
- reusable templates
- AI-oriented node primitives

That is the gap versus the current Dioxus test workflow demo, which is mostly a canvas interaction demo.

## High-value feature ideas

### 1. Real node library instead of generic nodes

Right now the Rust demo has visual node kinds:

- Trigger
- Data
- Agent
- Output

That is good for layout testing, but limited as a product surface.

Feature upgrade:

- node picker / insert menu
- concrete node types with distinct behavior
- categories such as:
  - input
  - llm
  - memory
  - tools
  - transforms
  - control flow
  - output

This is probably the single biggest product shift.

### 2. Node-side configuration UI

The Simple AI workflow experience appears to treat each node as something configurable, not just draggable.

Feature upgrade:

- click node to open config panel
- editable fields per node
- placeholders, prompts, labels, model choice, tool selection, output mapping
- validation messages at the node level

Your current inline label edit is a start, but it is not enough to feel like a workflow editor.

### 3. Handle-level semantics

One major product difference is that connections mean something.

Feature upgrade:

- named input/output handles
- different handle types
- only compatible handles can connect
- multiple outputs from one node
- branch-style output handles such as `success`, `error`, `true`, `false`

This makes the canvas feel like a real workflow system instead of freeform graph drawing.

### 4. Execution states on nodes and edges

This is one of the most useful features for an AI workflow UI.

Feature upgrade:

- idle / queued / running / success / error states
- active edge highlighting while execution progresses
- per-node duration
- last run timestamp
- error payload / logs for failed nodes

You already have a status demo. The next step is to make execution state first-class across the full workflow.

### 5. Run / preview / inspect workflow

Simple AI presents the workflow as something executable, not just editable.

Feature upgrade:

- Run button
- step-by-step execution mode
- run current graph with sample input
- inspector for node output
- execution log panel

This would dramatically increase the usefulness of the demo pages.

### 6. Templates and starter workflows

The page feels oriented around reusable AI workflow patterns.

Feature upgrade:

- starter templates
- "new from template"
- preset graphs for common tasks:
  - chat assistant
  - RAG flow
  - summarizer
  - classification
  - multi-step agent

This is high leverage because it shows users what the system is for immediately.

## Medium-value feature ideas

### 7. Better node content than title + description

Current nodes are presentation shells.

Feature upgrade:

- badges for model/provider/tool
- compact parameter summaries
- token or cost hints
- output schema preview
- warnings directly on card

The node card should explain what it does without opening a panel.

### 8. Workflow validation feedback

Users should know why a graph is incomplete.

Feature upgrade:

- missing required inputs
- disconnected required nodes
- invalid output mapping
- incompatible connection types
- empty prompt/config warnings

This should surface both globally and on the affected nodes.

### 9. Side panels for structure and inspection

A strong workflow product usually has more than the canvas.

Feature upgrade:

- left panel: node catalog
- right panel: selected node settings
- bottom panel: execution logs / outputs
- top toolbar: run, validate, zoom, reset, export

This makes the experience feel substantially more complete.

### 10. Branching and control-flow nodes

AI workflows become much more useful once control flow exists.

Feature upgrade:

- if/else node
- router node
- retry node
- parallel fan-out
- merge/join node

That moves the canvas from "pipeline" to "workflow engine".

## Lower-level but still valuable UX features

### 11. Searchable add-node command

Instead of only toolbar insertion:

- press a key or click "+"
- search nodes by name
- insert at cursor

This becomes important once the node library grows.

### 12. Per-node test input and preview

Helpful especially for prompts and transforms.

Feature upgrade:

- test a node in isolation
- preview transformed output
- inspect prompt variables before full run

### 13. Reusable subflows

If the product grows:

- group nodes into a reusable macro/subflow
- collapse/expand subflow
- reuse common patterns across workflows

### 14. Workflow metadata

Small but useful:

- workflow title
- description
- tags
- version / last updated
- run history summary

## Suggested priority for your Rust demo

If the goal is to make `src/domain/test` more product-like, I would prioritize:

1. Node library with concrete node types
2. Right-side configuration panel
3. Validation feedback on nodes/connections
4. Execution states and run simulation
5. Starter templates

That sequence gives the biggest jump in perceived capability.

## What to avoid

Do not spend too much time only improving canvas mechanics if the feature surface stays generic.

Examples of low-leverage-only work:

- more drag polish without richer nodes
- more edge styling without execution semantics
- more keyboard shortcuts without configuration UI

Those are useful, but they do not change the product meaningfully on their own.

## Bottom line

The main lesson from Simple AI at the feature level is:

the value is not the graph editor itself, but the fact that each node is a meaningful AI building block with configuration, validation, and runtime visibility.
