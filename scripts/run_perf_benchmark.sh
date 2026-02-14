#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${ROOT_DIR}/output_tests/perf"
OUT_FILE="${OUT_DIR}/latest.json"

mkdir -p "${OUT_DIR}"

INPUT_DIR="${1:-${ROOT_DIR}/tests/aaa}"
ITERATIONS="${2:-3}"
MAX_FILES="${3:-5}"

pushd "${ROOT_DIR}" >/dev/null
cargo run --release --example perf_benchmark -- \
  --input-dir "${INPUT_DIR}" \
  --iterations "${ITERATIONS}" \
  --max-files "${MAX_FILES}" | tee "${OUT_FILE}"
popd >/dev/null

echo "perf-result=${OUT_FILE}"
