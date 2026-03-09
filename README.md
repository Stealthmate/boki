# boki

TODO: simple description

## How to release a new version

1. Decide the next version number (e.g. `v1.2.3`).
2. Add a new section to the top of [`docs/release-notes.md`](./docs/release-notes.md), describing the changes from the previous version. Commit this.
3. Update the binary version in `Cargo.toml`. Commit this.
4. Create a tag with the specified version, e.g. `git tag -a v1.2.3 -m 'see docs/release-notes.md'` and push the tag.
