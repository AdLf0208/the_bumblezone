#!/usr/bin/env python3
"""The Bumblezone 0.1.2 spawner data contract: the nine biome files' spawners.

The artefact (not a name or a note) is the authority. This tool reads the nine
`data/the_bumblezone/worldgen/biome/*.json` `spawners` objects from the cached
jar and writes a checked-in snapshot of their VALUES. No jar byte is copied:
`tools/gen_spawner_data.py` emits typed Rust constants from the snapshot,
`tools/gen_biomes.py` emits our own 26.3-shape JSON, and
`src/runtime.rs` pins the parsed weights in unit tests.

    python3 tools/spawner_contract.py --write   # author tools/spawner_contract.json
    python3 tools/spawner_contract.py --check   # snapshot vs jar (exit 1 on drift)

Exit 0 when the snapshot and the jar agree; 1 otherwise, 2 if the jar is missing.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CAMPAIGN = ROOT.parent.parent
JAR = CAMPAIGN / "ports/_shared/licence-cache/the_bumblezone-7.17.3+1.21.1-neoforge.jar"
SNAPSHOT = ROOT / "tools/spawner_contract.json"
BIOME_DIR = "data/the_bumblezone/worldgen/biome/"
CATEGORIES = (
    "ambient",
    "axolotls",
    "creature",
    "misc",
    "monster",
    "underground_water_creature",
    "water_ambient",
    "water_creature",
)
ENTRY_FIELDS = ("type", "weight", "minCount", "maxCount")


def jar_facts() -> dict:
    with zipfile.ZipFile(JAR) as archive:
        names = sorted(
            n for n in archive.namelist()
            if n.startswith(BIOME_DIR) and n.endswith(".json")
        )
        biomes = {}
        for name in names:
            stem = name[len(BIOME_DIR):-5]
            root = json.loads(archive.read(name))
            spawners = root.get("spawners", {})
            missing = [c for c in CATEGORIES if c not in spawners]
            if missing:
                raise RuntimeError(f"{name}: spawners lacks categories {missing}")
            biomes[stem] = {
                category: [
                    {field: entry[field] for field in ENTRY_FIELDS}
                    for entry in spawners[category]
                ]
                for category in CATEGORIES
            }
    if len(biomes) != 9:
        raise RuntimeError(f"expected nine biome files, found {len(biomes)}")
    return {
        "jar_sha256": hashlib.sha256(JAR.read_bytes()).hexdigest(),
        "biome_count": len(biomes),
        "categories": list(CATEGORIES),
        "biomes": biomes,
    }


def write_snapshot() -> int:
    facts = jar_facts()
    facts["note"] = (
        "Authored from the cached jar's nine data/the_bumblezone/worldgen/biome/"
        "*.json spawners objects by tools/spawner_contract.py. VALUES ONLY "
        "(type, weight, minCount, maxCount): no jar byte is copied. "
        "tools/gen_spawner_data.py emits typed Rust; tools/gen_biomes.py emits the "
        "26.3 natural_mob_spawns shape; tools/check_provenance.py proves zero bytes."
    )
    SNAPSHOT.write_text(json.dumps(facts, indent=2, sort_keys=True) + "\n")
    total = sum(
        len(entries)
        for biome in facts["biomes"].values()
        for entries in biome.values()
    )
    print(f"wrote {SNAPSHOT.relative_to(ROOT)}: {len(facts['biomes'])} biomes, {total} entries")
    return 0


def check_tree() -> int:
    if not SNAPSHOT.is_file():
        print(f"FAIL {SNAPSHOT.relative_to(ROOT)}: missing", file=sys.stderr)
        return 1
    snapshot = json.loads(SNAPSHOT.read_text())
    try:
        facts = jar_facts()
    except Exception as exc:
        print(f"FAIL jar: {exc}", file=sys.stderr)
        return 1
    problems = []
    for key, want in sorted(facts.items()):
        if snapshot.get(key) != want:
            problems.append(f"{key}: snapshot != jar ({want!r})")
    for key in sorted(snapshot):
        if key not in facts and key != "note":
            problems.append(f"{key}: snapshot key has no jar evidence")
    if problems:
        for problem in problems:
            print(f"FAIL {problem}")
        print(f"spawner contract FAILED: {len(problems)} problem(s)")
        return 1
    total = sum(
        len(entries)
        for biome in facts["biomes"].values()
        for entries in biome.values()
    )
    print(
        f"OK    jar/snapshot agree on {facts['biome_count']} biomes, "
        f"{len(CATEGORIES)} categories, {total} entries; values only, no bytes"
    )
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--write", action="store_true")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    if not JAR.is_file():
        print(f"missing jar: {JAR}", file=sys.stderr)
        return 2
    if args.write:
        return write_snapshot()
    return check_tree()


if __name__ == "__main__":
    raise SystemExit(main())
