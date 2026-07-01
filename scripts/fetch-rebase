#!/usr/bin/env bash
set -euo pipefail

if ! git diff --quiet || ! git diff --cached --quiet; then
  echo "warning: working tree is dirty, skipping fetch-rebase" >&2
  exit 0
fi

echo "git fetch origin master"
git fetch origin master

echo "git fetch origin master --tags"
git fetch origin master --tags

echo "git rebase origin/master"
git rebase origin/master
