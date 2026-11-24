#!/usr/bin/env python3
"""The Bumblezone slice-1 data contract: dimension ids and dimension_type values.

Nothing from the jar is copied into the package. This tool reads the artefact as
*evidence* and writes a checked-in snapshot of the values the port declares in
`src/runtime.rs`; the Rust unit tests pin the same numbers. ARR assets are
regenerated separately (`tools/gen_icon.py`) and proved byte-free by
`tools/check_provenance.py`.

    python3 tools/carry.py --check   # data-contract gate (red before --write)
    python3 tools/carry.py --write   # author tools/dimension_contract.json

Exit 0 when the snapshot, the jar and the Rust constants agree; 1 otherwise.
"""

from __future__ import annotations

import argparse
import json
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CAMPAIGN = ROOT.parent.parent
JAR = CAMPAIGN / "ports/_shared/licence-cache/the_bumblezone-7.17.3+1.21.1-neoforge.jar"
SNAPSHOT = ROOT / "tools/dimension_contract.json"
RUNTIME = ROOT / "src/runtime.rs"
CLASS = "com/telepathicgrunt/the_bumblezone/modinit/BzDimension.class"

# The jar's own ids, read from its dimension file and its registration class'
# constant pool (no decompilation). They are the family this slice registers.
IDS = {
    "dimension": "the_bumblezone:the_bumblezone",
    "chunk_generator": "the_bumblezone:chunk_generator",
    "biome_source": "the_bumblezone:biome_source",
    "noise_settings": "the_bumblezone:bz_noise_settings",
}
# Keys of tools/dimension_contract.json that come from the jar's dimension_type.
TYPE_KEYS = (
    "min_y", "height", "logical_height", "ambient_light", "has_skylight",
    "has_ceiling", "infiniburn", "coordinate_scale",
    "monster_spawn_block_light_limit", "monster_spawn_light_level",
    "fixed_time", "has_raids", "respawn_anchor_works", "bed_works",
    "piglin_safe", "ultrawarm", "natural",
)


def jar_facts() -> dict:
    with zipfile.ZipFile(JAR) as archive:
        names = archive.namelist()
        dim_type = json.loads(archive.read("data/the_bumblezone/dimension_type/the_bumblezone.json"))
        dim = json.loads(archive.read("data/the_bumblezone/dimension/the_bumblezone.json"))
        class_pool = archive.read(CLASS).decode("latin-1")
    missing = [n for n in (CLASS, "data/the_bumblezone/dimension/the_bumblezone.json") if n not in names]
    if missing:
        raise RuntimeError(f"jar is missing {missing}")
    for token in ("chunk_generator", "biome_source", "the_bumblezone"):
        if token not in class_pool:
            raise RuntimeError(f"{CLASS} has no {token!r} in its constant pool")
    if dim.get("type") != IDS["dimension"]:
        raise RuntimeError(f"dimension type id is {dim.get('type')!r}")
    generator = dim.get("generator", {})
    if generator.get("type") != IDS["chunk_generator"]:
        raise RuntimeError(f"generator id is {generator.get('type')!r}")
    if generator.get("settings") != IDS["noise_settings"]:
        raise RuntimeError(f"noise settings id is {generator.get('settings')!r}")
    if generator.get("biome_source", {}).get("type") != IDS["biome_source"]:
        raise RuntimeError("biome source id differs from the registration class")
    facts = {key: dim_type[key] for key in TYPE_KEYS}
    facts.update(IDS)
    facts["main_biomes"] = generator["biome_source"]["main_biomes"]
    facts["jar_sha256"] = __import__("hashlib").sha256(JAR.read_bytes()).hexdigest()
    return facts


def write_snapshot() -> int:
    facts = jar_facts()
    facts["note"] = (
        "Authored from the cached jar's dimension/dimension_type JSON and the "
        "BzDimension class constant pool by tools/carry.py. Values only: no jar "
        "byte is copied into the package. The ids and numbers are what "
        "src/runtime.rs declares to Registrar::register_*."
    )
    SNAPSHOT.write_text(json.dumps(facts, indent=2, sort_keys=True) + "\n")
    print(f"wrote {SNAPSHOT.relative_to(ROOT)}: {len(facts)} keys")
    return 0


def check_tree() -> int:
    problems: list[str] = []
    if not SNAPSHOT.is_file():
        print(f"FAIL {SNAPSHOT.relative_to(ROOT)}: missing (run tools/carry.py --write)", file=__import__("sys").stderr)
        return 1
    snapshot = json.loads(SNAPSHOT.read_text())
    try:
        facts = jar_facts()
    except Exception as exc:  # a jar change must be a loud failure, never a silent pass
        print(f"FAIL jar: {exc}")
        return 1
    for key, want in sorted(facts.items()):
        if snapshot.get(key) != want:
            problems.append(f"{key}: snapshot {snapshot.get(key)!r} != jar {want!r}")
    for key in sorted(snapshot):
        if key not in facts and key != "note":
            problems.append(f"{key}: snapshot key has no jar evidence")

    # The Rust side must name the same ids and embed the same contract. The
    # package builds from src/ alone (tools/ is not packed), so the contract is
    # compiled into the source and checked here against the jar and the snapshot.
    if RUNTIME.is_file():
        rust = RUNTIME.read_text()
        for name, value in IDS.items():
            const = f'pub const {name.upper()}_ID: &str = "{value}";'
            if const not in rust:
                problems.append(f"src/runtime.rs: missing {const}")
        marker = "const CONTRACT_JSON: &str = r##\""
        if marker not in rust:
            problems.append("src/runtime.rs: no embedded CONTRACT_JSON")
        else:
            embedded = rust.split(marker, 1)[1].split('}"##;', 1)[0] + "}"
            try:
                parsed = json.loads(embedded)
            except ValueError as exc:
                problems.append(f"src/runtime.rs: CONTRACT_JSON does not parse ({exc})")
            else:
                for key, want in sorted(facts.items()):
                    if parsed.get(key) != want:
                        problems.append(
                            f"src/runtime.rs CONTRACT_JSON {key}: {parsed.get(key)!r} != jar {want!r}")
    else:
        problems.append("src/runtime.rs is missing")

    # The blocked families must not be present as data: the pin cannot apply
    # them without a level reset or a configured-feature registrar.
    for banned in ("data/minecraft", "data/the_bumblezone/worldgen/feature",
                   "data/the_bumblezone/dimension_type", "data/the_bumblezone/dimension"):
        if (ROOT / banned).exists():
            problems.append(f"{banned}: packed but blocked at this pin")

    if problems:
        for problem in problems:
            print(f"FAIL {problem}")
        print(f"data contract FAILED: {len(problems)} problem(s)")
        return 1
    print(f"OK    snapshot/jar agree on {len(facts)} keys; ids in src/runtime.rs; "
          "blocked families absent")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--write", action="store_true")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    if args.write:
        return write_snapshot()
    return check_tree()


if __name__ == "__main__":
    raise SystemExit(main())
