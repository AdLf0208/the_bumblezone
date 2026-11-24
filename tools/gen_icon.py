#!/usr/bin/env python3
"""Draw the port's monogram icon from its own palette — never the project art.

The upstream art is All Rights Reserved (LICENSE_ASSETS.txt in the jar), so the
registry icon is a monogram: a "B" for Bumblezone inside a hex cell drawn from
scratch. The same call is deterministic; `--check` re-draws and compares bytes.

    python3 tools/gen_icon.py            # write icon.png
    python3 tools/gen_icon.py --check    # assert it is unchanged
"""

from __future__ import annotations

import argparse
import hashlib
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[1]
ICON = ROOT / "icon.png"
SIZE = 128
BACKGROUND = (38, 23, 8, 255)
CELL = (242, 179, 35, 255)
RIM = (120, 76, 12, 255)
LETTER = (255, 244, 214, 255)


def draw() -> bytes:
    image = Image.new("RGBA", (SIZE, SIZE), BACKGROUND)
    pen = ImageDraw.Draw(image)
    # A honeycomb cell, generated here (six vertices, no upstream image).
    center = SIZE / 2
    radius = 56
    hexagons = [
        [
            (
                center + radius * __import__("math").cos(__import__("math").radians(60 * i - 30)),
                center + radius * __import__("math").sin(__import__("math").radians(60 * i - 30)),
            )
            for i in range(6)
        ]
    ]
    for outline in hexagons:
        pen.polygon(outline, fill=BACKGROUND, outline=RIM, width=6)
        inner = [
            (
                center + (radius - 9) * __import__("math").cos(__import__("math").radians(60 * i - 30)),
                center + (radius - 9) * __import__("math").sin(__import__("math").radians(60 * i - 30)),
            )
            for i in range(6)
        ]
        pen.line(inner + [inner[0]], fill=CELL, width=3, joint="curve")
    font = ImageFont.load_default(size=84)
    box = pen.textbbox((0, 0), "B", font=font)
    pen.text(
        (center - (box[2] - box[0]) / 2 - box[0], center - (box[3] - box[1]) / 2 - box[1]),
        "B",
        font=font,
        fill=LETTER,
    )
    import io

    buffer = io.BytesIO()
    image.save(buffer, format="PNG", optimize=True)
    return buffer.getvalue()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    raw = draw()
    if args.check:
        if not ICON.is_file():
            print(f"FAIL {ICON.name}: missing (run tools/gen_icon.py)")
            return 1
        if ICON.read_bytes() != raw:
            print(f"FAIL {ICON.name}: not the deterministic monogram")
            return 1
        print(f"OK    {ICON.name}: monogram, {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}")
        return 0
    ICON.write_bytes(raw)
    print(f"wrote {ICON.name}: {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
