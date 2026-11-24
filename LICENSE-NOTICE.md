# Licence notice — The Bumblezone port

**Upstream:** *The Bumblezone* by **TelepathicGrunt** —
<https://github.com/TelepathicGrunt/Bumblezone>.

The cached artefact is `the_bumblezone-7.17.3+1.21.1-neoforge.jar` (83,641,779 bytes,
SHA-256 `859d87b44d3d17c63f38317ac34d124e30b6de8c3043fa61ee987ff8b1f236d6`).

## The split, read from the artefact

- **Code: LGPL-3.0-only (rank 1).** The jar ships `LICENSE.txt` = "GNU LESSER GENERAL
  PUBLIC LICENSE, Version 3, 29 June 2007". `META-INF/neoforge.mods.toml` says
  `license="MIT"`, but the licence file inside the jar wins (ARCHITECTURE.md §8, the
  2026-11-19 ruling that names this mod). The complete corresponding source of this
  rewrite travels with the package under that licence; `LICENSE` is the canonical
  LGPL-3.0-only text from the system SPDX licence collection, **not** a copy of the
  jar's file.
- **Assets: All Rights Reserved (`assets-arr`).** The jar ships
  `LICENSE_ASSETS.txt` = "All Bumblezone Assets are ARR (All Rights Reserved)", with
  the author's explanation (bought/borrowed music, a texture lent from another mod,
  and the textures he made himself). **No upstream asset byte is packaged here.**
  Every visual in this port is generated from this port's own palette by
  `tools/gen_icon.py`; `tools/check_provenance.py` measures zero jar-derived bytes
  and writes `PROVENANCE.md`.
- The registry icon is a **monogram**, not the project art (PLAN.md §icons): the
  art is precisely what the ARR file reserves.

This is a clean-room Rust rewrite of the mod's observed behaviour and data, not a
translation of its Java code. It is local-only until publishing is approved for the
code; the ARR assets are never redistributed.
