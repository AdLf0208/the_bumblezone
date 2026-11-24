#!/usr/bin/env python3
"""Regenerate the nine Bumblezone biome definitions in the 26.3 codec shape.

The upstream biome JSON is ARR (`LICENSE_ASSETS.txt`), so no jar byte is carried.
This tool reads only *values* from `tools/biome_contract.json` (climate, colours,
sound) and writes our own 26.3-shape `data/the_bumblezone/worldgen/biome/*.json`
files: the shape the game's `BiomeInfo::from_json` decoder reads (measured in the
pin provider, see `evidence/the_bumblezone/biome-codec-gate.log`). Mob spawners,
carvers and configured features are deliberately empty: the configured-feature
registrar is absent at this pin (`blocked(configured-feature-not-registered)`)
and the spawner lists are a separate family.

    python3 tools/gen_biomes.py           # write the files
    python3 tools/gen_biomes.py --check   # assert the tree matches this generator
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CONTRACT = ROOT / "tools/biome_contract.json"
OUT = ROOT / "data/the_bumblezone/worldgen/biome"
STEPS = 7  # vanilla biome.features has 7 decoration steps


def colour(value: int) -> str:
    return f"#{value & 0xFFFFFF:06x}"


def definition(values: dict) -> dict:
    return {
        "attributes": {
            "minecraft:gameplay/natural_mob_spawns": {
                "argument": {"spawns_by_category": {}, "spawn_costs": {}},
                "modifier": "overlay",
            },
            "minecraft:gameplay/creature_world_gen_spawn_probability": values["creature_spawn_probability"],
            "minecraft:visual/sky_color": colour(values["sky"]),
            "minecraft:visual/fog_color": colour(values["fog"]),
            "minecraft:visual/water_fog_color": colour(values["water_fog"]),
        },
        # A list, not an object: the generator's carver decoder treats a single
        # object as one carver definition and demands its `type` field.
        "carvers": [],
        "downfall": values["downfall"],
        "effects": {
            "ambient_sound": values["ambient_sound"],
            "foliage_color": colour(values["foliage"]),
            "grass_color": colour(values["grass"]),
            "water_color": colour(values["water"]),
        },
        "features": [[] for _ in range(STEPS)],
        "has_precipitation": values["has_precipitation"],
        "temperature": values["temperature"],
    }


def render(values: dict) -> bytes:
    return (json.dumps(definition(values), indent=2, sort_keys=True) + "\n").encode("utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    contract = json.loads(CONTRACT.read_text())
    want = {f"{name}.json": render(v) for name, v in contract["biomes"].items()}
    if args.check:
        problems = []
        actual = {p.name: p.read_bytes() for p in OUT.glob("*.json")} if OUT.is_dir() else {}
        if set(actual) != set(want):
            problems.append(f"files: tree {sorted(actual)} != contract {sorted(want)}")
        for name, data in want.items():
            if actual.get(name) != data:
                problems.append(f"{name}: tree bytes differ from generator output")
        if problems:
            for p in problems:
                print(f"FAIL {p}", file=sys.stderr)
            return 1
        print(f"OK    {len(want)} biome files match the generator ({sum(map(len, want.values()))} bytes)")
        return 0
    OUT.mkdir(parents=True, exist_ok=True)
    for name, data in sorted(want.items()):
        (OUT / name).write_bytes(data)
    print(f"wrote {len(want)} biome files to {OUT.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
