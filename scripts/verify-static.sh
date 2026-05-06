#!/bin/sh
set -u

log_dir="$(mktemp -d "${TMPDIR:-/tmp}/verify-static.XXXXXX")"

cleanup() {
  rm -rf "$log_dir"
}

trap cleanup EXIT HUP INT TERM

run_stage() {
  stage="$1"
  shift
  log_file="$log_dir/$stage.log"

  "$@" >"$log_file" 2>&1
  status="$?"

  if [ "$status" -eq 0 ]; then
    printf 'verify %s ... ok\n' "$stage"
    return 0
  fi

  printf 'verify %s ... failed\n' "$stage"
  printf -- '----- %s output -----\n' "$stage"
  cat "$log_file"
  exit "$status"
}

run_stage fmt cargo fmt -- --check
run_stage clippy cargo clippy --all-targets -- -D warnings
run_stage test cargo test
run_stage docs-topology cargo run -- docs validate-topology
run_stage line-limits cargo run -- quality check-lines

printf 'verify pass\n'
