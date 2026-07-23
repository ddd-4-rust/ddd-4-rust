#!/usr/bin/env bash
set -euo pipefail

workspace_root=$(cd "$(dirname "$0")/.." && pwd)
registry_root=$(mktemp -d)
registry_port=8000
ktra_bin=${KTRA_BIN:-ktra}
ktra_pid=""

cleanup() {
    if [[ -n "$ktra_pid" ]]; then
        kill "$ktra_pid" 2>/dev/null || true
        wait "$ktra_pid" 2>/dev/null || true
    fi
    rm -rf "$registry_root"
}
trap cleanup EXIT

git init --bare --initial-branch=main "$registry_root/remote.git" >/dev/null
git init --initial-branch=main "$registry_root/seed" >/dev/null
git -C "$registry_root/seed" config user.name ddd4rust-ci
git -C "$registry_root/seed" config user.email ddd4rust-ci@example.invalid
printf '{"dl":"http://127.0.0.1:%s/dl","api":"http://127.0.0.1:%s"}\n' \
    "$registry_port" "$registry_port" > "$registry_root/seed/config.json"
git -C "$registry_root/seed" add config.json
git -C "$registry_root/seed" commit -m "Initialize temporary Cargo index" >/dev/null
git -C "$registry_root/seed" remote add origin "file://$registry_root/remote.git"
git -C "$registry_root/seed" push -u origin main >/dev/null

"$ktra_bin" \
    --address 127.0.0.1 \
    --remote-url "file://$registry_root/remote.git" \
    --local-path "$registry_root/index" \
    --db-dir-path "$registry_root/db" \
    --dl-dir-path "$registry_root/crates" \
    --cache-dir-path "$registry_root/cache" \
    --git-name ddd4rust-ci \
    --git-email ddd4rust-ci@example.invalid \
    > "$registry_root/ktra.log" 2>&1 &
ktra_pid=$!

for _ in {1..30}; do
    if curl --fail --silent "http://127.0.0.1:$registry_port/me" >/dev/null; then
        break
    fi
    if ! kill -0 "$ktra_pid" 2>/dev/null; then
        sed -n '1,160p' "$registry_root/ktra.log" >&2
        exit 1
    fi
    sleep 1
done
curl --fail --silent "http://127.0.0.1:$registry_port/me" >/dev/null

registry_token=$(curl --fail --silent \
    -X POST \
    -H 'content-type: application/json' \
    -d '{"password":"temporary-ci-password"}' \
    "http://127.0.0.1:$registry_port/ktra/api/v1/new_user/ci" \
    | python3 -c 'import json,sys; print(json.load(sys.stdin)["token"])')

export CARGO_REGISTRIES_DDD4RUST_LOCAL_INDEX="file://$registry_root/remote.git"
export CARGO_REGISTRIES_DDD4RUST_LOCAL_TOKEN="$registry_token"

cd "$workspace_root"
for package in \
    ddd-4-rust-core \
    ddd-4-rust-codegen-api \
    ddd-4-rust-codegen-processor \
    ddd-4-rust-serde \
    ddd-4-rust-esc \
    ddd-4-rust-test
do
    cargo publish --locked --allow-dirty --registry ddd4rust-local -p "$package"
done

consumer="$registry_root/consumer"
mkdir -p "$consumer/src"
printf 'fn main() {}\n' > "$consumer/src/main.rs"
printf '%s\n' \
    '[package]' \
    'name = "ddd4rust-registry-consumer"' \
    'version = "0.1.0"' \
    'edition = "2024"' \
    '' \
    '[dependencies]' \
    'ddd-4-rust-core = { version = "=0.7.0", registry = "ddd4rust-local" }' \
    'ddd-4-rust-codegen-api = { version = "=0.7.0", registry = "ddd4rust-local" }' \
    'ddd-4-rust-codegen-processor = { version = "=0.7.0", registry = "ddd4rust-local" }' \
    'ddd-4-rust-serde = { version = "=0.7.0", registry = "ddd4rust-local", features = ["xml"] }' \
    'ddd-4-rust-esc = { version = "=0.7.0", registry = "ddd4rust-local" }' \
    'ddd-4-rust-test = { version = "=0.7.0", registry = "ddd4rust-local" }' \
    > "$consumer/Cargo.toml"
cargo check --manifest-path "$consumer/Cargo.toml"
