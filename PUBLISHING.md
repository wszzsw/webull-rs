# Publishing Checklist for webull-rs

This document outlines the steps to publish the webull-rs crate to crates.io.

## Pre-Publication Checklist

- [ ] Update version number in Cargo.toml
- [ ] Update CHANGELOG.md with the latest changes
- [ ] Ensure all examples are working and up-to-date
- [ ] Run all tests to ensure they pass
- [ ] Check code coverage and ensure it meets the target threshold
- [ ] Run cargo clippy to check for any linting issues
- [ ] Run cargo fmt to ensure consistent code formatting
- [ ] Verify documentation is complete and accurate
- [ ] Check that all public APIs have proper documentation
- [ ] Ensure README.md is up-to-date
- [ ] Verify that all dependencies are at their latest compatible versions
- [ ] Check for any security vulnerabilities in dependencies
- [ ] Ensure license information is correct
- [ ] Verify that the crate builds with --no-default-features
- [ ] Check that the crate builds on all supported platforms

## Publication Steps

1. **Final Version Check**
   ```bash
   cargo check
   cargo test
   cargo clippy
   cargo fmt --check
   ```

2. **Documentation Check**
   ```bash
   cargo doc --no-deps
   ```

3. **Dry Run**
   ```bash
   cargo publish --dry-run
   ```

4. **Package Verification**
   ```bash
   cargo package --list
   ```

5. **Publish to crates.io**
   ```bash
   cargo login
   cargo publish
   ```

6. **Post-Publication Verification**
   - [ ] Verify the crate is available on crates.io
   - [ ] Check that documentation is properly displayed on docs.rs
   - [ ] Verify that the version badge in README.md is updated
   - [ ] Create a GitHub release with the release notes

## Version Bump for Development

After publishing, bump the version number in Cargo.toml to the next development version:

```toml
[package]
name = "webull-rs"
version = "0.1.1-dev"  # Next development version
```

## Notes

- Remember to follow semantic versioning (MAJOR.MINOR.PATCH)
- For pre-releases, use versions like "0.1.0-alpha.1", "0.1.0-beta.1", etc.
- When publishing a new major version (1.0.0, 2.0.0, etc.), ensure all breaking changes are documented
