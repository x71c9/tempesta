#!/usr/bin/env bash
#
# Configure branch protection on `master` so that the PR Validation checks
# must pass before a PR can be merged. This makes "PR Validation" propaedeutic
# to "Release on PR Merge": GitHub disables the merge button until the required
# checks are green, so a release can never run on a failing build.
#
# The required check names below must match the job `name:` fields in
# .github/workflows/pr-validation.yml.
#
# Requirements: `gh` CLI authenticated with `repo` scope and admin on the repo.

set -euo pipefail

# Infer repo from git remote if not set explicitly
_git_repo() {
  git remote get-url origin 2>/dev/null \
    | sed -E 's|.*github\.com[:/]||' | sed 's|\.git$||'
}
REPO="${REPO:-$(_git_repo)}"
if [[ -z "$REPO" ]]; then
  echo "error: could not infer repo from git remote. Set REPO=owner/name explicitly." >&2
  exit 1
fi
BRANCH="${BRANCH:-master}"

# Toggles (override via env):
#   STRICT=true|false        require branch up to date before merging
#   ENFORCE_ADMINS=true|false enforce the rule on admins too
STRICT="${STRICT:-true}"
ENFORCE_ADMINS="${ENFORCE_ADMINS:-false}"

# Required status checks. Must match job `name:` values in pr-validation.yml.
CONTEXTS=(
  "Run Tests"
  "Validate PR Title (Future Squash Commit)"
)

echo "Configuring branch protection for ${REPO}@${BRANCH}"
echo "  strict (up to date):  ${STRICT}"
echo "  enforce on admins:    ${ENFORCE_ADMINS}"
echo "  required checks:"
for c in "${CONTEXTS[@]}"; do echo "    - ${c}"; done
echo

# Build the JSON contexts array from the bash array.
contexts_json=$(printf '%s\n' "${CONTEXTS[@]}" | \
  awk 'BEGIN{printf "["} {printf "%s\"%s\"", (NR>1?",":""), $0} END{printf "]"}')

gh api -X PUT "repos/${REPO}/branches/${BRANCH}/protection" --input - <<JSON
{
  "required_status_checks": {
    "strict": ${STRICT},
    "contexts": ${contexts_json}
  },
  "enforce_admins": ${ENFORCE_ADMINS},
  "required_pull_request_reviews": null,
  "restrictions": null
}
JSON

echo
echo "Done. Branch protection applied to ${REPO}@${BRANCH}."
