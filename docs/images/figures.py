# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (C) 2026 Agent-IX
"""The filament-specification documentation figures, rendered by the docs-figures skill.

Only the corpus chart is defined here: its bar count and lengths follow the spec/ tree,
which docs/images/figures.json cannot yet express (data-driven geometry). The other
figures are defined in docs/images/figures.json and rendered by `make figures`.

Regenerate from the repository root with the agent-ix/dev-tools skill:

    python3 -m pip install fonttools
    python3 <dev-tools>/skills/docs-figures/scripts/render.py docs/images/figures.py

Add --check to confirm the committed SVGs are current. Every count in the figures
is read from this repository's tree: spec artifacts, vector files and test
functions are counted here, never typed.
"""

from pathlib import Path

from docs_figures import FONTS, SMALL, TITLE, Figure  # noqa: F401

STYLE = "ix-docs"

ROOT = Path(__file__).resolve().parents[2]

# ---- repository data ---------------------------------------------------------


def spec_dirs():
    """Artifact counts per spec/ directory, largest kinds in directory order."""
    out = []
    for d in sorted((ROOT / "spec").iterdir()):
        if d.is_dir() and d.name not in ("reviews", "subsystems"):
            out.append((d.name, len(list(d.rglob("*.md")))))
    return out


# ---- figures -----------------------------------------------------------------


def corpus():
    """Spec artifacts per kind, counted from the spec/ tree."""
    dirs = spec_dirs()
    total = sum(n for _, n in dirs)
    f = Figure(
        "corpus",
        980,
        260,
        f"The spec/ tree holds {total} authored artifacts in {len(dirs)} kinds: "
        + ", ".join(f"{name} {n}" for name, n in dirs)
        + ".",
    )
    f.text(0, 16, f"{total} SPEC ARTIFACTS IN {len(dirs)} KINDS", "muted", "mono", SMALL)
    top = max(n for _, n in dirs)
    for i, (name, n) in enumerate(dirs):
        y = 36 + i * 24
        f.text(0, y + 11, name, "ink", "mono", SMALL)
        f.rect(150, y, 700, 14, "muted", stroke=False, opacity=0.18)
        f.bar(150, y, 700 * n / top, 14, "code")
        f.text(150 + 700 * n / top + 10, y + 11, str(n), "muted", "mono", SMALL)
    return f


FIGURES = [corpus]
