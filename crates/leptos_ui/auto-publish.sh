#!/bin/bash
set -e

CRATE_NAME="leptos_ui"

# Check for CARGO_TOKEN environment variable
[ -n "$CARGO_TOKEN" ] || error "CARGO_TOKEN environment variable not found"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

error() { echo -e "${RED}ERROR: $1${NC}"; exit 1; }
info() { echo -e "${GREEN}$1${NC}"; }
warn() { echo -e "${YELLOW}$1${NC}"; }

# Basic checks
command -v git >/dev/null || error "git not found"
command -v cargo >/dev/null || error "cargo not found" 
git rev-parse --git-dir >/dev/null 2>&1 || error "Not a git repo"
[ "$(git rev-parse --abbrev-ref HEAD)" = "master" ] || error "Must be on master branch"

# Skip uncommitted changes check for CI/CD environments
# (GitHub Actions checkout may create temporary changes)
info "Skipping uncommitted changes check for automated publishing"

info "Auto-publishing $CRATE_NAME crate..."

# Pull latest
warn "Pulling latest..."
git pull origin master

# Get current version and increment patch
CURRENT=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
[ -n "$CURRENT" ] || error "Can't read version from Cargo.toml"

IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT"
NEW_VERSION="$MAJOR.$MINOR.$((PATCH + 1))"

info "Bumping $CURRENT → $NEW_VERSION"

# Update version
sed -i.bak "s/^version = \"$CURRENT\"/version = \"$NEW_VERSION\"/" Cargo.toml
rm -f Cargo.toml.bak

# Verify and test
UPDATED=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
[ "$UPDATED" = "$NEW_VERSION" ] || error "Version update failed"


# Commit and push
warn "Committing and pushing..."
git add Cargo.toml
git commit -m "📦 $CRATE_NAME: v$NEW_VERSION (auto)"
git push origin master

# Setup auth and publish
export CARGO_REGISTRY_TOKEN="$CARGO_TOKEN"
info "Using CARGO_TOKEN from environment"

# Disable sccache for publishing (not installed in CI environment)
export RUSTC_WRAPPER=""

warn "Publishing to crates.io..."
cargo publish --allow-dirty

info "✅ Successfully published $NEW_VERSION to crates.io!"