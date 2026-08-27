#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="${RUNNER_TEMP:-${TMPDIR:-/tmp}}/regelverket-packaging"
rm -rf "$OUT"
mkdir -p "$OUT/go" "$OUT/rust"

case "${RUNNER_OS:-$(uname -s)}" in
  Windows|MINGW*|MSYS*|CYGWIN*) exe=".exe"; platform="windows" ;;
  macOS|Darwin) exe=""; platform="macos" ;;
  Linux) exe=""; platform="linux" ;;
  *) echo "unsupported runner OS" >&2; exit 2 ;;
esac

printf '==> Go release build (%s)\n' "$platform"
(
  cd "$ROOT/spikes/go"
  go test ./...
  go build -trimpath -ldflags='-s -w' -o "$OUT/go/regelverket$exe" .
)

printf '==> Rust release build (%s)\n' "$platform"
(
  cd "$ROOT/spikes/rust"
  cargo test --locked
  cargo build --locked --release
  cp "target/release/regelverket-rust-spike$exe" "$OUT/rust/regelverket$exe"
)

for impl in go rust; do
  test -s "$OUT/$impl/regelverket$exe"
  printf '%s\n' "$impl:$platform:regelverket$exe" > "$OUT/$impl/MANIFEST.txt"
  if [[ "$platform" == windows ]]; then
    (cd "$OUT/$impl" && tar -a -cf "$OUT/regelverket-$impl-$platform.zip" "regelverket$exe" MANIFEST.txt)
    test -s "$OUT/regelverket-$impl-$platform.zip"
  else
    (cd "$OUT/$impl" && tar -czf "$OUT/regelverket-$impl-$platform.tar.gz" regelverket MANIFEST.txt)
    test -s "$OUT/regelverket-$impl-$platform.tar.gz"
  fi
done

printf 'packaging spike verification: PASS (%s)\n' "$platform"
