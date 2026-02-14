#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RESULT_FILE="${1:-${ROOT_DIR}/output_tests/perf/latest.json}"
THRESHOLD_MS="${2:-15.0}"

if [[ ! -f "${RESULT_FILE}" ]]; then
  echo "perf-result-file-not-found: ${RESULT_FILE}" >&2
  exit 1
fi

AVG_MS="$(sed -n 's/.*"avg_ms":\([0-9.]*\).*/\1/p' "${RESULT_FILE}")"
if [[ -z "${AVG_MS}" ]]; then
  echo "failed-to-parse-avg-ms: ${RESULT_FILE}" >&2
  exit 1
fi

if awk "BEGIN{exit !(${AVG_MS} <= ${THRESHOLD_MS})}"; then
  echo "perf-threshold-pass avg_ms=${AVG_MS} threshold_ms=${THRESHOLD_MS}"
  exit 0
fi

echo "perf-threshold-fail avg_ms=${AVG_MS} threshold_ms=${THRESHOLD_MS}" >&2
exit 1
