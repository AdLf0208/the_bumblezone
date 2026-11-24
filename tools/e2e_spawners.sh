#!/usr/bin/env bash
# In-game proof for the_bumblezone 0.1.2: the nine biome files' mob spawner lists.
#
# GAME/CTL come from `eval "$(ports/tools/launcher-game.sh env the_bumblezone)"`.
#
#   tools/e2e_spawners.sh                      # modded: category weights -> PASS EXIT:0
#   GAME=$CONTROL_GAME tools/e2e_spawners.sh   # control: command unknown -> FAIL EXIT:1
#   SELFTEST=1 tools/e2e_spawners.sh           # expects an empty vanilla list -> FAIL EXIT:1
#
# Every command's exact text and JSON reply are printed; assertions parse the
# value, never the absence of an error. `E2E_TMP` (never /tmp) holds scratch data.
set -uo pipefail
export LC_ALL=C

CAMPAIGN=/home/adlf/Desktop/all_the_mods_10_rust_rewrite
CTL=${CTL:-}
GAME=${GAME:-}
RES=${RES:-${HORIZON_RES:-/home/adlf/Desktop/horizon/.local/horizon-client/26.3}}
E2E_TMP=${E2E_TMP:-/tmp/opencode}
SELFTEST=${SELFTEST:-0}
PROBE="/the_bumblezone spawners the_bumblezone:hive_pillar"
# The jar's hive_pillar spawners: creature 50+20+5+5 = 80, monster 20+2+1+2+1 = 26.
WANT_CREATURE=80
WANT_MONSTER=26

[ -n "$CTL" ] && [ -n "$GAME" ] || { echo "missing CTL/GAME: eval \"\$(ports/tools/launcher-game.sh env the_bumblezone)\"" >&2; exit 1; }
[ -x "$CTL" ] || { echo "missing horizon-ctl: $CTL" >&2; exit 1; }
[ -x "$GAME" ] || { echo "missing horizon-client: $GAME" >&2; exit 1; }
[ -d "$RES/assets" ] || { echo "missing prepared resources: $RES" >&2; exit 1; }

say() { echo "== $* =="; }
fail() { echo "FAIL: $*" >&2; exit 1; }

SCRATCH=$(mktemp -d "$E2E_TMP/the_bumblezone-spawners.XXXXXX")
export HORIZON_HOME=$SCRATCH/game
export HORIZON_LAUNCHER_HOME=$SCRATCH/launcher
export XDG_DATA_HOME=$SCRATCH/data
export XDG_CONFIG_HOME=$SCRATCH/config
export HORIZON_NO_AUDIO=1
export HORIZON_LAUNCHER_NO_SELF_UPDATE=1
SOCK=$E2E_TMP/the_bumblezone-spawners-$$.sock
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
    --game "$GAME" --world BumblezoneSpawnersE2E --seed 20261105 --creative --headless --wait \
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

say "the spawner readout prints the parsed category weights"
# `ctl chat` is the player-chat path the mod's command table hears; `ctl command`
# runs the console path and would bypass `run_mod_command`.
ctl chat "$PROBE" >/dev/null
lines=""
for i in $(seq 1 40); do
    lines=$(ctl messages 10 | jq -r '.chat[]?')
    if grep -q 'hive_pillar spawns:' <<<"$lines" || grep -qi 'Unknown or incomplete command' <<<"$lines"; then
        break
    fi
    ctl wait 2 >/dev/null
done
echo "probe $PROBE"
echo "reply: $lines"

if [ "$SELFTEST" = 1 ]; then
    # Inverted: vanilla's empty natural_mob_spawns prints no category weight.
    # The carried contract prints them, so this leg must fail.
    if grep -q 'weight=' <<<"$lines"; then
        fail "SELFTEST: the spawner readout named category weights (an empty vanilla list was expected): $lines"
    fi
    fail "SELFTEST: no category weight was printed (unexpected for this package): $lines"
fi

if grep -qi 'Unknown or incomplete command' <<<"$lines"; then
    fail "control: the unmodded game answered Unknown or incomplete command, as expected: $lines"
fi
grep -q 'the_bumblezone:hive_pillar spawns:' <<<"$lines" \
    || fail "the reply did not name the probed biome: $lines"
grep -q "creature weight=$WANT_CREATURE" <<<"$lines" \
    || fail "creature weight $WANT_CREATURE missing from the readout: $lines"
grep -q "monster weight=$WANT_MONSTER" <<<"$lines" \
    || fail "monster weight $WANT_MONSTER missing from the readout: $lines"

say "differential: an unknown biome is refused by the readout"
ctl chat "/the_bumblezone spawners the_bumblezone:no_such_biome" >/dev/null
unknown=""
for i in $(seq 1 20); do
    unknown=$(ctl messages 10 | jq -r '.chat[]?')
    if grep -q 'no_such_biome spawns: unknown biome' <<<"$unknown"; then break; fi
    ctl wait 1 >/dev/null
done
echo "unknown-biome reply: $unknown"
grep -q 'no_such_biome spawns: unknown biome' <<<"$unknown" \
    || fail "an unknown biome was not refused by the readout: $unknown"

echo "PASS: the carried spawner contract prints creature weight=$WANT_CREATURE and monster weight=$WANT_MONSTER for the_bumblezone:hive_pillar"
