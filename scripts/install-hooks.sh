#!/usr/bin/env bash
#
# Install the repo's git hooks. Points git at scripts/hooks/ via core.hooksPath
# so the hooks are version-controlled and shared with everyone who clones.
#
# Run once after cloning:  ./scripts/install-hooks.sh

set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
hooks_dir="${repo_root}/scripts/hooks"

chmod +x "${hooks_dir}"/* 2>/dev/null || true

git -C "${repo_root}" config core.hooksPath scripts/hooks

echo "Installed git hooks from scripts/hooks (core.hooksPath set)."
echo "Hooks active:"
ls -1 "${hooks_dir}"
