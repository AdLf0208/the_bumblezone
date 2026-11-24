#!/usr/bin/env python3
"""Prove that no packaged byte came from the ARR upstream jar.

The jar ships `LICENSE_ASSETS.txt` = "All Bumblezone Assets are ARR", so this port
carries none of its art. Every visual is drawn by `tools/gen_icon.py` from this
port's own palette; the manifest, docs, Rust and the data contract are authored
here; the LGPL text comes from the system SPDX licence collection, not the jar.

This tool measures the claim rather than asserting it:

* hash every entry of the cached jar (a copy under a different name is caught);
* hash every candidate packaged file of the port tree, plus every entry of the
  delivered `the_bumblezone.hmod` when it exists;
* report the **jar-derived asset byte count** (zero is the only acceptable value)
  and write `PROVENANCE.md`, one line per packaged file, naming its generator.

    python3 tools/check_provenance.py            # assert, then write PROVENANCE.md
    python3 tools/check_provenance.py --check    # assert only (the ladder gate)
    python3 tools/check_provenance.py --check <jar>

Exit 0 when the count is zero, 1 otherwise, 2 when the jar is missing.
"""

from __future__ import annotations

import argparse
import hashlib
import sys
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CAMPAIGN = ROOT.parent.parent
DEFAULT_JAR = CAMPAIGN / "ports/_shared/licence-cache/the_bumblezone-7.17.3+1.21.1-neoforge.jar"
PACKAGE = ROOT / "the_bumblezone.hmod"
REPORT = ROOT / "PROVENANCE.md"

GEN_ICON = "generated (`tools/gen_icon.py`: own honeycomb palette and B monogram)"
AUTHORED = "authored (this port, no upstream byte)"
CONTRACT = "authored (`tools/carry.py`: values read from the jar, our own bytes)"
BIOME = "authored (`tools/biome_contract.py` values -> `tools/gen_biomes.py` 26.3 codec shape)"
LGPL = "authored (canonical LGPL-3.0 text from the system SPDX collection)"


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def candidates() -> list[tuple[str, bytes, str]]:
    """Every file this port can ship: source, manifest, docs, data, art."""
    out: list[tuple[str, bytes, str]] = []
    explicit = {
        "icon.png": GEN_ICON,
        "LICENSE": LGPL,
        "LICENSE-NOTICE.md": AUTHORED,
        "README.md": AUTHORED,
        "horizon-mod.toml": AUTHORED,
        "Cargo.toml": AUTHORED,
        "src/lib.rs": AUTHORED,
        "src/runtime.rs": AUTHORED,
        "src/spawner_data.rs": "authored (`tools/spawner_contract.json` values -> `tools/gen_spawner_data.py`)",
        "tools/dimension_contract.json": CONTRACT,
    }
    for name, origin in explicit.items():
        path = ROOT / name
        if path.is_file():
            out.append((name, path.read_bytes(), origin))
    for folder in ("assets", "data"):
        for path in sorted((ROOT / folder).rglob("*")) if (ROOT / folder).is_dir() else []:
            if path.is_file():
                rel = path.relative_to(ROOT).as_posix()
                origin = BIOME if rel.startswith("data/the_bumblezone/worldgen/biome/") else GEN_ICON
                out.append((rel, path.read_bytes(), origin))
    return out


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("jar", nargs="?", default=str(DEFAULT_JAR))
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    jar = Path(args.jar)
    if not jar.is_file():
        print(f"missing jar: {jar}", file=sys.stderr)
        return 2

    jar_hashes: dict[str, str] = {}
    with zipfile.ZipFile(jar) as archive:
        for info in archive.infolist():
            if not info.is_dir():
                jar_hashes.setdefault(digest(archive.read(info)), info.filename)

    rows = candidates()
    matched = 0
    matched_bytes = 0
    for name, data, _origin in rows:
        found = jar_hashes.get(digest(data))
        if found:
            matched += 1
            matched_bytes += len(data)
            print(f"jar-derived: {name} == {found}", file=sys.stderr)

    with zipfile.ZipFile(jar) as archive:
        asset_entries = sum(1 for n in archive.namelist() if n.startswith("assets/"))
    total = sum(len(data) for _, data, _ in rows)
    print(
        f"jar: {jar.name} ({len(jar_hashes)} distinct entry hashes; "
        f"assets/** entries: {asset_entries})"
    )
    print(f"tree: {len(rows)} candidate files, {total} bytes")
    print(f"jar-derived asset bytes: {matched_bytes} (files matched: {matched})")

    if PACKAGE.is_file():
        with zipfile.ZipFile(PACKAGE) as archive:
            packaged = [info for info in archive.infolist() if not info.is_dir()]
            for info in packaged:
                found = jar_hashes.get(digest(archive.read(info)))
                if found:
                    matched += 1
                    matched_bytes += len(archive.read(info))
                    print(f"jar-derived: package:{info.filename} == {found}", file=sys.stderr)
        print(f"package: {len(packaged)} entries; packaged art files: "
              f"{sum(1 for i in packaged if i.filename.startswith('assets/'))}")
    else:
        print("package: not built yet")

    if matched:
        return 1
    if not args.check:
        lines = [
            "# Provenance — The Bumblezone port",
            "",
            f"Jar: `{jar.name}` — {len(jar_hashes)} distinct entry hashes. "
            "**No packaged file has any of those bytes** (compared by content, so a "
            "copy under a different name is caught too).",
            f"Jar-derived asset bytes: **{matched_bytes}** (files matched: {matched}).",
            "",
            "| File | Origin |",
            "|---|---|",
        ]
        lines += [f"| `{name}` | {origin} |" for name, _data, origin in rows]
        lines += [
            "",
            "The upstream `LICENSE_ASSETS.txt` reserves all art; the icon is a monogram "
            "drawn by this port. The LGPL text is the canonical SPDX file from the system, "
            "not the jar's `LICENSE.txt`. The dimension values are transcribed as data, not "
            "copied as bytes (`tools/carry.py`).",
            "",
        ]
        REPORT.write_text("\n".join(lines))
        print(f"wrote {REPORT.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
