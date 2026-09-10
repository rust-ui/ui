# Learnings

One line per learning. Format: `YYYY-MM-DD [domain] avoid X, do Y, because Z`.

New captures land in **Inbox** (not yet reviewed). Move a line up to
**Confirmed** once validated as worth keeping. Delete from Inbox what you do
not want.

## Confirmed

- 2026-09-10 [rust] avoid hand-written `match self { Enum::Variant => "Variant", ... }` for enum-to-string, do derive `strum::IntoStaticStr` / `strum::AsRefStr` (+ `#[strum(serialize_all=...)]` or per-variant `#[strum(to_string=...)]`) and call `.into()` / `.as_ref()`, because single source of truth, no variant/string drift, less boilerplate. Keep a manual match only when one enum needs several different string projections (e.g. `dot_color()` + `text_color()` + `label()`) or computed values; then still use `Self::Variant` arms (clippy::use_self). Precedent: src/markdown/highlight_language.rs, use_workflow.rs EdgeStyle.

## Inbox

