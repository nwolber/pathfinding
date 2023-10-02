#! /bin/sh
#
# Usage: ./release.sh [arguments to cargo release]

set -e

git changelog -n CHANGELOG.md
changelog=$(mktemp)
awk '/^n/,/^v/{if(/^ /)print}' < CHANGELOG.md > "$changelog"
git commit -am "Prepare ChangeLog for next release"
echo "Changelog that will be used for this release:"
echo "---"
cat "$changelog"
echo "---"
cargo release --execute "$@"
tag=$(git tag --list --sort=-v:refname | head -n 1)
gh release create $tag -F "$changelog"
rm "$changelog"
