#!/usr/bin/env python3
"""Build CLIPPY_TRACKER.md (compact) from CLIPPY_FULL_DUMP.txt (verbose clippy scan).

Run from the RUST-UI workspace root. See strict-clippy-burndown.md.
"""
import re, collections, os

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
src = os.path.join(ROOT, "CLIPPY_FULL_DUMP.txt")
dst = os.path.join(ROOT, "CLIPPY_TRACKER.md")

lines = open(src, encoding="utf-8", errors="replace").read().splitlines()

items = []  # (lint, loc, msg)
cur_msg = cur_loc = None
for ln in lines:
    m = re.match(r'^warning: (.*)', ln)
    if m:
        cur_msg, cur_loc = m.group(1).strip(), None
        continue
    m = re.match(r'^\s*-->\s+(\S+)', ln)
    if m and cur_msg is not None and cur_loc is None:
        cur_loc = m.group(1)
        continue
    m = re.search(r'index\.html#([a-z_]+)', ln)
    if m and cur_msg is not None:
        items.append((m.group(1), cur_loc or "(no location)", cur_msg))
        cur_msg = cur_loc = None

by_lint = collections.defaultdict(list)
for lint, loc, msg in items:
    by_lint[lint].append((loc, msg))

out, total = [], 0
for lint in sorted(by_lint, key=lambda k: -len(by_lint[k])):
    seen, rows = set(), []
    for loc, msg in by_lint[lint]:
        if (loc, msg) in seen:
            continue
        seen.add((loc, msg))
        rows.append((loc, msg))
    total += len(rows)
    out.append(f"## {lint} ({len(rows)})\n")
    out.append(f"<!-- {rows[0][1]} -->\n")
    out += [f"- [ ] {loc}\n" for loc, _ in sorted(rows)]
    out.append("\n")

header = (
    "# Clippy strict tracker\n\n"
    f"Total unique sites: {total}. Source: CLIPPY_FULL_DUMP.txt (regenerate with the SOP scan).\n"
    "Tick/delete lines as fixed. Suggestion detail: `grep -A20 '<path:line>' CLIPPY_FULL_DUMP.txt`.\n\n"
)
open(dst, "w", encoding="utf-8").write(header + "".join(out))
print(f"wrote {dst}, unique sites: {total}")
