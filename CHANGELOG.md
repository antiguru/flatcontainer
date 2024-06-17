# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.1](https://github.com/antiguru/flatcontainer/compare/v0.4.0...v0.4.1) - 2024-06-17

### Other
- Add missing Ord and ReserveItems impls ([#39](https://github.com/antiguru/flatcontainer/pull/39))
- Huffman container ([#20](https://github.com/antiguru/flatcontainer/pull/20))
- Fix warning on Rust 1.79 ([#38](https://github.com/antiguru/flatcontainer/pull/38))
- Move complex tests to separate folder ([#34](https://github.com/antiguru/flatcontainer/pull/34))

## [0.3.2](https://github.com/antiguru/flatcontainer/compare/v0.3.1...v0.3.2) - 2024-05-28

### Other
- Thinking about relating owned types and read items ([#31](https://github.com/antiguru/flatcontainer/pull/31))
- Introduce reborrow to enable lifetime variance ([#32](https://github.com/antiguru/flatcontainer/pull/32))

## [0.3.1](https://github.com/antiguru/flatcontainer/compare/v0.3.0...v0.3.1) - 2024-05-24

### Other
- Update recommended version to 0.3 ([#29](https://github.com/antiguru/flatcontainer/pull/29))

## [0.3.0](https://github.com/antiguru/flatcontainer/compare/v0.2.0...v0.3.0) - 2024-05-24

### Other
- Replace CopyOnto by Push ([#28](https://github.com/antiguru/flatcontainer/pull/28))
- Fix bench, add to ci

## [0.2.0](https://github.com/antiguru/flatcontainer/compare/v0.1.0...v0.2.0) - 2024-03-13

### Changes
- Merge pull request [#23](https://github.com/antiguru/flatcontainer/pull/23) from antiguru/slice_implementations
- Rename CopyRegion to OwnedRegion and relax trait bounds
- Remove CopyOnto requirement for ReadItem
- Remove index parameter from columns region
