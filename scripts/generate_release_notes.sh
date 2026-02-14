#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
pushd "${ROOT_DIR}" >/dev/null

TO_REF="${1:-HEAD}"
FROM_REF="${2:-}"
OUT_FILE="${3:-${ROOT_DIR}/output_tests/release_notes.md}"

if [[ -z "${FROM_REF}" ]]; then
  if [[ "${TO_REF}" == "HEAD" ]]; then
    FROM_REF="$(git describe --tags --abbrev=0 HEAD^ 2>/dev/null || true)"
  else
    FROM_REF="$(git describe --tags --abbrev=0 "${TO_REF}^" 2>/dev/null || true)"
  fi
fi

mkdir -p "$(dirname "${OUT_FILE}")"

if [[ -n "${FROM_REF}" ]]; then
  RANGE="${FROM_REF}..${TO_REF}"
else
  RANGE="${TO_REF}"
fi

{
  echo "# Release Notes"
  echo
  echo "- to: \`${TO_REF}\`"
  if [[ -n "${FROM_REF}" ]]; then
    echo "- from: \`${FROM_REF}\`"
  else
    echo "- from: \`(initial history)\`"
  fi
  echo

  declare -a sections=(
    "feat:Features"
    "fix:Fixes"
    "refactor:Refactors"
    "test:Tests"
    "docs:Docs"
    "chore:Chores"
  )

  for entry in "${sections[@]}"; do
    prefix="${entry%%:*}"
    title="${entry#*:}"
    echo "## ${title}"
    matches="$(git log --pretty=format:'- `%h` %s' "${RANGE}" --grep="^${prefix}" -i || true)"
    if [[ -n "${matches}" ]]; then
      echo "${matches}"
    else
      echo "- (none)"
    fi
    echo
  done

  echo "## Other"
  other="$(git log --pretty=format:'- `%h` %s' "${RANGE}" \
    --invert-grep \
    --grep='^feat' \
    --grep='^fix' \
    --grep='^refactor' \
    --grep='^test' \
    --grep='^docs' \
    --grep='^chore' -i || true)"
  if [[ -n "${other}" ]]; then
    echo "${other}"
  else
    echo "- (none)"
  fi
} > "${OUT_FILE}"

popd >/dev/null
echo "release-notes=${OUT_FILE}"
