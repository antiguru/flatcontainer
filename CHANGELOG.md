# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.0](https://github.com/antiguru/flatcontainer/compare/v0.5.0...v0.6.0) - 2024-07-29

### Other
- Bump CodSpeedHQ/action from 2 to 3 ([#63](https://github.com/antiguru/flatcontainer/pull/63))
- Support bencher and codspeed ([#60](https://github.com/antiguru/flatcontainer/pull/60))
- Rename offset to index to better capture its meaning ([#55](https://github.com/antiguru/flatcontainer/pull/55))
- Only output throughput if BYTES is set ([#56](https://github.com/antiguru/flatcontainer/pull/56))
- Simplify offset iterators and enable clone ([#53](https://github.com/antiguru/flatcontainer/pull/53))
- use OffsetContainer iter ([#52](https://github.com/antiguru/flatcontainer/pull/52))
- Rename slice_copy to slice_owned ([#51](https://github.com/antiguru/flatcontainer/pull/51))
- Rename CopyIter to PushIter ([#50](https://github.com/antiguru/flatcontainer/pull/50))
- Separate data storage ([#19](https://github.com/antiguru/flatcontainer/pull/19))

## [0.5.0](https://github.com/antiguru/flatcontainer/compare/v0.4.1...v0.5.0) - 2024-06-26

### Other
- Rename Containerized to RegionPreference and add owned type ([#47](https://github.com/antiguru/flatcontainer/pull/47))
- Use vectors as regions ([#46](https://github.com/antiguru/flatcontainer/pull/46))
- Efficient cloning of regions and flat stack ([#45](https://github.com/antiguru/flatcontainer/pull/45))
- Use OffsetOptimized in consecutive offset pairs ([#43](https://github.com/antiguru/flatcontainer/pull/43))
- Add reserve items to consecutive offset pairs ([#42](https://github.com/antiguru/flatcontainer/pull/42))
- Improve GatCow test ([#41](https://github.com/antiguru/flatcontainer/pull/41))

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
