#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

OBSERVED="$ROOT/fixtures/observed-state/avkroken-dumpen-v0.yaml"
CONFIG_NOOP="$ROOT/fixtures/config/avkroken-dumpen-sequential-slots-v0.yaml"
CONFIG_UPDATE="$ROOT/fixtures/config/avkroken-dumpen-sequential-slots-add-check-v0.yaml"

printf '==> Go tests and host build\n'
(
  cd "$ROOT/spikes/go"
  go test ./...
  go build -trimpath -o "$TMP/regelverket-go-spike" .
  go run . "$OBSERVED" "$CONFIG_NOOP" > "$TMP/go-noop.json"
  go run . "$OBSERVED" "$CONFIG_UPDATE" > "$TMP/go-update.json"
)

printf '==> Rust tests and host build\n'
(
  cd "$ROOT/spikes/rust"
  export CARGO_TARGET_DIR="$TMP/cargo-target"
  cargo test --locked
  cargo build --locked --release
  cargo run --locked --quiet -- "$OBSERVED" "$CONFIG_NOOP" > "$TMP/rust-noop.json"
  cargo run --locked --quiet -- "$OBSERVED" "$CONFIG_UPDATE" > "$TMP/rust-update.json"
)

printf '==> Semantic parity\n'
python3 - "$TMP" <<'PY'
import json
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
for case in ("noop", "update"):
    go = json.loads((root / f"go-{case}.json").read_text())
    rust = json.loads((root / f"rust-{case}.json").read_text())
    assert go == rust, (case, go, rust)

noop = json.loads((root / "go-noop.json").read_text())
assert noop["plan"]["no_changes"] is True
assert noop["plan"]["operations"] == []

update = json.loads((root / "go-update.json").read_text())
assert update["plan"]["no_changes"] is False
assert update["plan"]["operations"][0]["add"] == ["dependency-review"]
assert update["plan"]["operations"][0].get("remove", []) == []
PY

printf '==> Reproducibility\n'
(
  cd "$ROOT/spikes/go"
  go run . "$OBSERVED" "$CONFIG_UPDATE" > "$TMP/go-update-second.json"
)
(
  cd "$ROOT/spikes/rust"
  export CARGO_TARGET_DIR="$TMP/cargo-target"
  cargo run --locked --quiet -- "$OBSERVED" "$CONFIG_UPDATE" > "$TMP/rust-update-second.json"
)
cmp "$TMP/go-update.json" "$TMP/go-update-second.json"
cmp "$TMP/rust-update.json" "$TMP/rust-update-second.json"

printf 'technology spike verification: PASS\n'
