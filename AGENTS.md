


- **Changelog + versioning**: keep `CHANGELOG_DEV.md` up to date (Keep a Changelog format, entries land under `## [Unreleased]`). Semver tags are `vMAJOR.MINOR.PATCH` (annotated tags, current: `v0.1.0`), separate from the `deploy_prod_v2_*` timestamp tags. When `[Unreleased]` has grown enough to be worth cutting, tell me and propose the next version, then wait for me to confirm before renaming the section, tagging, and pushing.


### Self improving 

- When I correct a behavior/pattern/preference (not a one-off fact) and you judge it will recur, append one bullet under `## Inbox` in `__SKILLS_LEARNINGS/LEARNINGS.md` (`YYYY-MM-DD [domain] avoid X, do Y, because Z`) and mirror it to auto-memory as `feedback`. You decide, no keyword. Then print: `📝 learning saved: "<one-line>" (say "drop it" to undo)`. Skip: project trivia, anything already enforced by lint/tsconfig/biome/CI, low-confidence guesses. `learn this` forces it.
