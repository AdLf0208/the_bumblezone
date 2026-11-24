#!/usr/bin/env python3
"""The Bumblezone 0.1.1 biome data contract: the jar's nine worldgen/biome ids.

The artefact (not a name or a note) is the authority: this tool reads the nine
`data/the_bumblezone/worldgen/biome/*.json` files and their grouping tags from
the cached jar and checks that (a) the checked-in snapshot carries the same
values, and (b) `src/runtime.rs` declares the same nine ids and the same seven
BzBiomeSource layer families. VALUES are compared, never copied into the
package; the JSON the package ships is written by `tools/gen_biomes.py`.

    python3 tools/biome_contract.py --check

Exit 0 when the jar, the snapshot and the Rust constants agree; 1 otherwise.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CAMPAIGN = ROOT.parent.parent
JAR = CAMPAIGN / "ports/_shared/licence-cache/the_bumblezone-7.17.3+1.21.1-neoforge.jar"
SNAPSHOT = ROOT / "tools/biome_contract.json"
RUNTIME = ROOT / "src/runtime.rs"
SOURCE_CLASS = "com/telepathicgrunt/the_bumblezone/worldgen/dimension/BzBiomeSource.class"
BIOME_DIR = "data/the_bumblezone/worldgen/biome/"
TAG_DIR = "data/the_bumblezone/tags/worldgen/biome/"


def jar_facts() -> dict:
    with zipfile.ZipFile(JAR) as archive:
        names = set(archive.namelist())
        ids = sorted(
            n[len(BIOME_DIR):-5]
            for n in names
            if n.startswith(BIOME_DIR) and n.endswith(".json")
        )
        tags = {}
        for n in names:
            if n.startswith(TAG_DIR) and n.endswith(".json") and "/has_structure/" not in n:
                data = json.loads(archive.read(n))
                tags[n[len(TAG_DIR):-5]] = [
                    v.removeprefix("the_bumblezone:") for v in data.get("values", [])
                ]
        pool = archive.read(SOURCE_CLASS).decode("latin-1")
    # The exact family names, read from the class constant pool (no decompilation).
    families = sorted(set(re.findall(r"BzBiome[A-Za-z]*Layer", pool)))
    return {
        "ids": ids,
        "tags": tags,
        "families": families,
        "jar_sha256": hashlib.sha256(JAR.read_bytes()).hexdigest(),
    }


def check_tree() -> int:
    problems: list[str] = []
    if not SNAPSHOT.is_file():
        print(f"FAIL {SNAPSHOT.relative_to(ROOT)}: missing", file=sys.stderr)
        return 1
    snap = json.loads(SNAPSHOT.read_text())
    try:
        facts = jar_facts()
    except Exception as exc:
        print(f"FAIL jar: {exc}", file=sys.stderr)
        return 1

    if snap.get("jar_sha256") != facts["jar_sha256"]:
        problems.append(f"jar_sha256: snapshot {snap.get('jar_sha256')} != {facts['jar_sha256']}")
    jar_ids = ["the_bumblezone:" + i for i in facts["ids"]]
    want_order = ["the_bumblezone:" + i for i in snap["registry_order"]]
    if sorted(jar_ids) != sorted(want_order):
        problems.append(f"biome ids: snapshot {sorted(want_order)} != jar {sorted(jar_ids)}")
    if snap.get("biome_count") != len(jar_ids):
        problems.append(f"biome_count: snapshot {snap.get('biome_count')} != jar {len(jar_ids)}")
    if set(snap.get("biomes", {})) != set(facts["ids"]):
        problems.append("biomes: snapshot keys != jar biome ids")
    for tag, values in facts["tags"].items():
        if snap.get("tags", {}).get(tag) != values:
            problems.append(f"tag {tag}: snapshot {snap.get('tags', {}).get(tag)} != jar {values}")
    # Layer families: the class constant pool names each one; the snapshot lists them.
    for family in snap.get("layer_families", []):
        if family not in facts["families"]:
            problems.append(f"layer family {family}: not found in {SOURCE_CLASS}")

    # The Rust side must name the same ids (in order) and the same families.
    if not RUNTIME.is_file():
        problems.append("src/runtime.rs missing")
    else:
        rust = RUNTIME.read_text()
        for i, id_ in enumerate(want_order):
            marker = f'"{id_}"'
            if marker not in rust:
                problems.append(f"src/runtime.rs: missing biome id {marker}")
        for family in snap.get("layer_families", []):
            if family not in rust:
                problems.append(f"src/runtime.rs: missing layer family {family}")

    if problems:
        for p in problems:
            print(f"FAIL {p}")
        print(f"biome contract FAILED: {len(problems)} problem(s)")
        return 1
    print(f"OK    jar/snapshot agree on {len(jar_ids)} biomes, {len(facts['tags'])} tags, "
          f"{len(facts['families'])} layer families; ids + families present in src/runtime.rs")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    parser.parse_args()
    return check_tree()


if __name__ == "__main__":
    raise SystemExit(main())
