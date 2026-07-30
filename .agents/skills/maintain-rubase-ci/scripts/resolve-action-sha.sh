#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 ]]; then
  echo "usage: $0 OWNER/REPOSITORY TAG" >&2
  exit 2
fi

repository=$1
tag=${2#refs/tags/}

if ! command -v gh >/dev/null 2>&1; then
  echo "error: GitHub CLI (gh) is required" >&2
  exit 1
fi

object=$(
  gh api "repos/${repository}/git/ref/tags/${tag}" \
    --jq '.object | [.type, .sha] | @tsv'
)
IFS=$'\t' read -r object_type object_sha <<<"${object}"

for _ in {1..8}; do
  case "${object_type}" in
    commit)
      printf '%s\n' "${object_sha}"
      exit 0
      ;;
    tag)
      object=$(
        gh api "repos/${repository}/git/tags/${object_sha}" \
          --jq '.object | [.type, .sha] | @tsv'
      )
      IFS=$'\t' read -r object_type object_sha <<<"${object}"
      ;;
    *)
      echo "error: unsupported Git object type: ${object_type}" >&2
      exit 1
      ;;
  esac
done

echo "error: tag indirection exceeded eight levels" >&2
exit 1
