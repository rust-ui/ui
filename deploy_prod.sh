#!/bin/bash

cd "$(dirname "$0")"

# Exit on any error
set -e

echo "🔒 Pre-flight checks..."
BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [ "$BRANCH" != "main" ]; then
  echo "❌ Must be on main branch (currently on '$BRANCH')"
  exit 1
fi
if ! git diff --quiet || ! git diff --cached --quiet; then
  echo "❌ Uncommitted changes detected — commit or stash before deploying"
  exit 1
fi
echo "✅ On main, working tree clean"

echo "🔍 Checking for typos..."
typos .
echo "✅ No typos found"

echo "🎨 Formatting..."
cargo fmt --all
if ! git diff --quiet; then
  echo "📝 Formatting changes detected, committing..."
  git add -A
  git commit -m "fmt: auto-format before deploy"
  git push origin main
fi
echo "✅ Formatting OK"

echo "📎 Running clippy..."
cargo clippy --all-features
echo "✅ Clippy OK"

echo "🔒 Security audit..."
cargo audit
echo "✅ No vulnerabilities found"

echo "🧪 Running tests..."
cargo nextest run
echo "✅ All tests passed"

# Create timestamped deploy tag
TAG_NAME="deploy_$(date +'%Y/%m/%d_%Hh%Mm%Ss')"

echo "Creating tag: $TAG_NAME"
git tag $TAG_NAME

echo "Pushing tag to origin..."
git push origin $TAG_NAME

echo "Triggering deployment workflow..."
gh workflow run prod-vps.yml --repo rust-ui/dioxus-ui

echo "✅ Deploy tag created and workflow triggered!"
echo "Tag: $TAG_NAME"
