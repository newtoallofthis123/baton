#!/usr/bin/env bash
set -euo pipefail

# Minimal stand-in for the future signoff CLI: run the PR-tier checks
# locally, then attest them as commit statuses on the pushed HEAD.

fail() {
  echo "signoff: $*" >&2
  exit 1
}

command -v git >/dev/null || fail "git not found"
command -v gh >/dev/null || fail "gh not found"
command -v just >/dev/null || fail "just not found"

cd "$(git rev-parse --show-toplevel)"

[ -z "$(git status --porcelain)" ] || fail "working tree is not clean — commit or stash first"

started=$SECONDS

echo "==> format (just fmt-check)"
just fmt-check
echo "==> lint (just lint)"
just lint

[ -z "$(git status --porcelain)" ] || fail "working tree changed during the run — not attesting"

duration=$((SECONDS - started))
sha=$(git rev-parse HEAD)
user=$(git config user.name)

echo "==> pushing $sha"
git push

for check in format lint; do
  gh api --method POST "repos/{owner}/{repo}/statuses/$sha" \
    -f state=success \
    -f context="signoff/$check" \
    -f description="$user signed off locally (${duration}s)" >/dev/null
  echo "==> attested signoff/$check on ${sha:0:12}"
done

echo "signoff complete: $user, ${duration}s"
