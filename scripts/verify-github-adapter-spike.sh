#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP="$(mktemp -d)"
SERVER_PID=""
cleanup() {
  if [[ -n "$SERVER_PID" ]]; then
    kill "$SERVER_PID" 2>/dev/null || true
  fi
  rm -rf "$TMP"
}
trap cleanup EXIT

printf '==> Build Go adapter\n'
(
  cd "$ROOT/spikes/github-adapter/go"
  go test ./...
  go build -trimpath -o "$TMP/github-adapter-go" .
)

printf '==> Build Rust adapter\n'
(
  cd "$ROOT/spikes/github-adapter/rust"
  export CARGO_TARGET_DIR="$TMP/cargo-target"
  cargo test --locked
  cargo build --locked
  cp "$CARGO_TARGET_DIR/debug/github-adapter-rust-spike" "$TMP/github-adapter-rust"
)

printf '==> Verify request headers and status classification\n'
python3 - "$TMP/port" <<'PY' &
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
import sys

port_file = Path(sys.argv[1])

class Handler(BaseHTTPRequestHandler):
    def log_message(self, *_):
        pass

    def do_GET(self):
        required = {
            "Accept": "application/vnd.github+json",
            "X-GitHub-Api-Version": "2026-03-10",
            "Authorization": "Bearer spike-token",
        }
        for name, value in required.items():
            if self.headers.get(name) != value:
                self.send_response(400)
                self.end_headers()
                return
        if not self.headers.get("User-Agent", "").startswith("regelverket-"):
            self.send_response(400)
            self.end_headers()
            return
        status = int(self.path.rsplit("/", 1)[-1])
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.end_headers()
        if status == 200:
            self.wfile.write(b'{"full_name":"Avkroken/Regelverket"}')

server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
port_file.write_text(str(server.server_port))
server.serve_forever()
PY
SERVER_PID=$!
for _ in $(seq 1 100); do
  [[ -s "$TMP/port" ]] && break
  sleep 0.05
done
[[ -s "$TMP/port" ]]
BASE="http://127.0.0.1:$(cat "$TMP/port")"

export GITHUB_TOKEN=spike-token
for status in 200 401 403 404 422 429 500; do
  "$TMP/github-adapter-go" "$BASE/status/$status" > "$TMP/go-$status.json"
  "$TMP/github-adapter-rust" "$BASE/status/$status" > "$TMP/rust-$status.json"
  cmp "$TMP/go-$status.json" "$TMP/rust-$status.json"
done

python3 - "$TMP" <<'PY'
import json
import pathlib
import sys
root = pathlib.Path(sys.argv[1])
expected = {
    200: "ok",
    401: "authentication_failed",
    403: "permission_denied",
    404: "not_found_or_inaccessible",
    422: "validation_failed",
    429: "rate_limited",
    500: "github_service_error",
}
for status, classification in expected.items():
    result = json.loads((root / f"go-{status}.json").read_text())
    assert result["status"] == status
    assert result["class"] == classification
    if status == 200:
        assert result["repository"] == "Avkroken/Regelverket"
PY

printf '==> Verify live read-only GitHub request\n'
unset GITHUB_TOKEN
LIVE="https://api.github.com/repos/Avkroken/Regelverket"
"$TMP/github-adapter-go" "$LIVE" > "$TMP/go-live.json"
"$TMP/github-adapter-rust" "$LIVE" > "$TMP/rust-live.json"
cmp "$TMP/go-live.json" "$TMP/rust-live.json"
python3 - "$TMP/go-live.json" <<'PY'
import json
import sys
result = json.load(open(sys.argv[1]))
assert result == {
    "status": 200,
    "class": "ok",
    "repository": "Avkroken/Regelverket",
}
PY

printf 'github adapter spike verification: PASS\n'
