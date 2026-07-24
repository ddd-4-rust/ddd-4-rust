#!/usr/bin/env python3
"""Fail unless every instrumented source line in LCOV has hit count > 0.

cargo-llvm-cov's summary "Lines" metric can stay slightly below 100% on Rust
code because brace / derive / async regions are counted even when every DA
entry is covered. This script enforces true executable-line coverage via LCOV
`DA:` records under `**/src/**`.
"""

from __future__ import annotations

import argparse
import sys
from collections import defaultdict
from pathlib import Path


def main() -> int:
    """Parse LCOV and exit non-zero when any src line has zero hits."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("lcov", type=Path, help="Path to lcov.info")
    args = parser.parse_args()

    uncovered: dict[str, list[int]] = defaultdict(list)
    current: str | None = None
    for raw in args.lcov.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if line.startswith("SF:"):
            current = line[3:]
        elif line.startswith("DA:") and current is not None:
            line_no, hits = line[3:].split(",", 1)
            if hits == "0" and "/src/" in current and "/tests/" not in current:
                uncovered[current].append(int(line_no))
        elif line == "end_of_record":
            current = None

    if not uncovered:
        print("LCOV executable src coverage: 100% (no DA:0 under **/src/**)")
        return 0

    print("Uncovered executable src lines (DA:0):", file=sys.stderr)
    for path, lines in sorted(uncovered.items()):
        print(f"  {path}: {lines}", file=sys.stderr)
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
