#!/usr/bin/env bash
# In-game proof for The Bumblezone slice 1: the dimension and its two Java ids.
#
# GAME/CTL come from `eval "$(ports/tools/launcher-game.sh env the_bumblezone)"`.
# CONTROL_GAME is the same Nightly without the package, so the control leg
# differs in the package alone.
#
#   tools/e2e.sh                      # modded: ids + a mod biome in the readout, block in level
#   GAME=$CONTROL_GAME tools/e2e.sh   # control: ids unknown, command unknown -> FAIL EXIT:1
#   SELFTEST=1 tools/e2e.sh           # inverted: expects minecraft:plains -> FAIL EXIT:1
#   BLOCK_SELFTEST=1 tools/e2e.sh     # inverted block assertion   -> FAIL EXIT:1
set -uo pipefail
export LC_ALL=C

CAMPAIGN=/home/adlf/Desktop/all_the_mods_10_rust_rewrite
PORT=$CAMPAIGN/ports/the_bumblezone
CTL=${CTL:-}
GAME=${GAME:-}
RES=${RES:-${HORIZON_RES:-/home/adlf/Desktop/horizon/.local/horizon-client/26.3}}
E2E_TMP=${E2E_TMP:-/tmp/opencode}
SELFTEST=${SELFTEST:-0}
BLOCK_SELFTEST=${BLOCK_SELFTEST:-0}
DIMENSION=the_bumblezone:the_bumblezone
GENERATOR=the_bumblezone:chunk_generator
BIOME_SOURCE=the_bumblezone:biome_source
PROBE="/the_bumblezone dimension 0 8 0 20261105"

[ -n "$CTL" ] && [ -n "$GAME" ] || { echo "missing CTL/GAME: eval \"\$(ports/tools/launcher-game.sh env the_bumblezone)\"" >&2; exit 1; }
[ -x "$CTL" ] || { echo "missing horizon-ctl: $CTL" >&2; exit 1; }
[ -x "$GAME" ] || { echo "missing horizon-client: $GAME" >&2; exit 1; }
[ -d "$RES/assets" ] || { echo "missing prepared resources: $RES" >&2; exit 1; }

say() { echo "== $* =="; }
fail() { echo "FAIL: $*" >&2; exit 1; }

SCRATCH=$(mktemp -d "$E2E_TMP/the_bumblezone-e2e.XXXXXX")
export HORIZON_HOME=$SCRATCH/game
export HORIZON_LAUNCHER_HOME=$SCRATCH/launcher
export XDG_DATA_HOME=$SCRATCH/data
export XDG_CONFIG_HOME=$SCRATCH/config
export HORIZON_NO_AUDIO=1
export HORIZON_LAUNCHER_NO_SELF_UPDATE=1
SOCK=$E2E_TMP/the_bumblezone-e2e-$$.sock
mkdir -p "$SCRATCH" "$(dirname "$SOCK")" "$XDG_CONFIG_HOME"
rm -f "$SOCK"
cd "$SCRATCH"

ctl() { "$CTL" --socket "$SOCK" "$@"; }
cleanup() {
    rc=$?
    ctl quit >/dev/null 2>&1 || true
    sleep 1
    rm -rf "$SCRATCH" || true
    rm -f "$SOCK"
    echo "EXIT:$rc"
}
trap cleanup EXIT

say "launch (headless, creative, seed 20261105)"
if ! "$CTL" --socket "$SOCK" launch \
    --game "$GAME" --world BumblezoneE2E --seed 20261105 --creative --headless --wait \
    --resources "$RES" --game-dir "$SCRATCH/game" \
    > "$SCRATCH/launch.json" 2> "$SCRATCH/launch.err"; then
    echo "launch reply: $(cat "$SCRATCH/launch.json" 2>/dev/null)" >&2
    tail -25 "$SCRATCH/game/logs/latest.log" >&2 || true
    fail "the game did not launch"
fi
jq -c '{dimension, creative, ok: .launch.ok}' "$SCRATCH/launch.json" || true
ctl wait-ready 180 >/dev/null || fail "the game did not become ready"
state=$(ctl state)
jq -r '{dimension, screen: .screen, server: .server.dimension}' <<<"$state"
[ "$(jq -r '.screen' <<<"$state")" = "playing" ] || fail "not in the world"
echo "start dimension: $(jq -r '.dimension' <<<"$state")"

say "the registered level takes a block (an id only this package produces)"
reply=$(ctl command "execute in $DIMENSION run forceload add 8 8" 2>&1 | tr -d '\n')
echo "execute in $DIMENSION run forceload: $reply"
if grep -qi "unknown dimension" <<<"$reply"; then
    # Control (unmodded) leg: the mod command must be unknown too, not just the
    # dimension. Prove the readout id is refused before failing the control.
    ctl chat "$PROBE" >/dev/null
    c=""
    for i in $(seq 1 20); do
        c=$(ctl messages 10 | jq -r '.chat[]?')
        if grep -qi "Unknown or incomplete command" <<<"$c"; then break; fi
        ctl wait 1 >/dev/null
    done
    echo "control probe $PROBE: $c"
    grep -qi "Unknown or incomplete command" <<<"$c" \
        || fail "control: the mod command was accepted: $c"
    fail "$DIMENSION is unknown to this game (control command refused as expected): $reply"
fi
ready=0
for i in $(seq 1 60); do
    reply=$(ctl command "execute in $DIMENSION if block 8 8 8 minecraft:air run say BZ_LOADED_$i" 2>&1 | tr -d '\n')
    if grep -q "BZ_LOADED_$i" <<<"$reply"; then ready=1; break; fi
    ctl wait 5 >/dev/null
done
[ "$ready" = 1 ] || fail "the $DIMENSION chunk at 8,8 never loaded"

reply=$(ctl command "execute in $DIMENSION run setblock 8 8 8 minecraft:glass" 2>&1 | tr -d '\n')
echo "execute in $DIMENSION run setblock: $reply"
grep -qi "unknown dimension" <<<"$reply" && fail "$DIMENSION is unknown to this game: $reply"
ctl wait 4 >/dev/null
reply=$(ctl command "execute in $DIMENSION if block 8 8 8 minecraft:glass run say BZ_GLASS_OK" 2>&1 | tr -d '\n')
echo "read back: $reply"
if [ "$BLOCK_SELFTEST" = 1 ]; then
    grep -q "BZ_GLASS_OK" <<<"$reply" \
        && fail "BLOCK_SELFTEST: the glass is present; the inverted assertion expected air"
    echo "BLOCK_SELFTEST: no glass (unexpected)"
else
    grep -q "BZ_GLASS_OK" <<<"$reply" || fail "the glass written in $DIMENSION does not read back: $reply"
fi

say "the readout prints all three registered ids"
# `ctl chat` is the player-chat path the mod's command table hears; `ctl command`
# runs the console path and would bypass `run_mod_command`.
ctl chat "/the_bumblezone dimension" >/dev/null
lines=""
for i in $(seq 1 40); do
    lines=$(ctl messages 10 | jq -r '.chat[]?')
    if grep -q 'biome source the_bumblezone' <<<"$lines"; then break; fi
    ctl wait 2 >/dev/null
done
echo "readout: $lines"

say "the probed cell names a mod biome, not a vanilla stand-in"
ctl chat "$PROBE" >/dev/null
biome_lines=""
for i in $(seq 1 40); do
    biome_lines=$(ctl messages 10 | jq -r '.chat[]?')
    if grep -q 'biome the_bumblezone:' <<<"$biome_lines"; then break; fi
    ctl wait 2 >/dev/null
done
echo "biome readout: $biome_lines"
if [ "$SELFTEST" = 1 ]; then
    # Inverted: the old/four-stand-in source answered minecraft:plains here.
    grep -q 'biome minecraft:plains' <<<"$biome_lines" \
        || fail "SELFTEST: the probed cell named a mod biome, not minecraft:plains: $biome_lines"
    echo "SELFTEST: minecraft:plains present (unexpected)"
else
    grep -q 'biome the_bumblezone:' <<<"$biome_lines" \
        || fail "the probed cell did not name a mod biome: $biome_lines"
    if grep -q 'biome minecraft:' <<<"$biome_lines"; then
        fail "the probed cell named a vanilla stand-in: $biome_lines"
    fi
fi

if [ "$SELFTEST" = 1 ]; then
    # Inverted: the package's own ids must NOT be printed. They are, so this leg fails.
    grep -q "the_bumblezone:no_such_dimension" <<<"$lines" \
        || fail "SELFTEST: the readout named the real ids instead of the fake one: $lines"
    echo "SELFTEST: fake id present (unexpected)"
else
    grep -q "$DIMENSION" <<<"$lines" || fail "the readout did not name the dimension: $lines"
    grep -q "$GENERATOR" <<<"$lines" || fail "the readout did not name the generator: $lines"
    grep -q "$BIOME_SOURCE" <<<"$lines" || fail "the readout did not name the biome source: $lines"
    grep -q 'min_y=0 height=256' <<<"$lines" || fail "the readout did not name the jar's dimension_type: $lines"
fi

say "differential: an unregistered dimension id is refused"
reply=$(ctl command "execute in the_bumblezone:no_such_dimension run setblock 8 8 8 minecraft:glass" 2>&1 | tr -d '\n')
echo "unknown dimension: $reply"
grep -qi "unknown dimension" <<<"$reply" || fail "an unknown dimension id was accepted: $reply"

echo "PASS: the modded game registers $DIMENSION, $GENERATOR, $BIOME_SOURCE, and the probed cell names a mod biome"
