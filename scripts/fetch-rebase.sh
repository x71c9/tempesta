#!/usr/bin/env bash

set -euo pipefail

if ! git diff --quiet 2>/dev/null || ! git diff --cached --quiet 2>/dev/null; then
  echo "warning: dirty git tree — skipping fetch-rebase" >&2
  exit 0
fi

git fetch origin master
git fetch origin master --tags
git rebase origin/master
