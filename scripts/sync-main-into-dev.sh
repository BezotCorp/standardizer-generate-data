#!/usr/bin/env bash
set -euo pipefail

REPO="${GITHUB_REPOSITORY}"
SYNC_BRANCH="sync/main-into-dev"

# dev is normally ahead of main, so comparing SHAs is not enough: only skip
# when dev already contains every commit of main.
COMMITS_MISSING_IN_DEV=$(gh api "repos/$REPO/compare/dev...main" --jq .ahead_by)

if [ "$COMMITS_MISSING_IN_DEV" = "0" ]; then
  echo "dev already contains main — nothing to do."
  exit 0
fi

# An open sync PR is left strictly untouched: no branch deletion or recreation,
# no new PR, no conflict resolution. A human has to deal with it.
OPEN_SYNC_PR=$(gh pr list --repo "$REPO" \
  --head "$SYNC_BRANCH" --base dev --state open \
  --json url --jq '.[0].url // empty')

if [ -n "$OPEN_SYNC_PR" ]; then
  echo "::error::A sync PR is already open ($OPEN_SYNC_PR) while dev is missing $COMMITS_MISSING_IN_DEV commit(s) from main. Left untouched: resolve or close it manually."
  exit 1
fi

# No open sync PR: any existing $SYNC_BRANCH is stale and is recreated from the
# current main by create-branch.sh.
./scripts/create-branch.sh "$SYNC_BRANCH" main

./scripts/open-and-wait-pr-merge.sh \
  "$SYNC_BRANCH" dev \
  "Sync main into dev" \
  "chore: sync main into dev"
