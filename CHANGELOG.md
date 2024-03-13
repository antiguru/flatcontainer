# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/antiguru/flatcontainer/compare/v0.1.0...v0.2.0) - 2024-03-13

### Other
- Merge pull request [#23](https://github.com/antiguru/flatcontainer/pull/23) from antiguru/slice_implementations
- Rename CopyRegion to OwnedRegion and relax trait bounds
- Remove CopyOnto requirement for ReadItem
- Remove index parameter from columns region
- Add tests and cleanup minor findings
- Merge pull request [#16](https://github.com/antiguru/flatcontainer/pull/16) from antiguru/improvements
- Merge pull request [#17](https://github.com/antiguru/flatcontainer/pull/17) from antiguru/release-plz
- Integrate release-plz
- Documentation updates
- Fix warnings
- Update CI integration
- Relax Rust version to 1.65
- Heap size
- Remove staging vector in codec
- StringRegion generic over inner region
- Code movement, documentation,
- Explicit column iterator; support copying iterators to columns
- Relax paste version, make columns region public
- Dictionary codec plus supporting infrastructure
- Fix tests for no default features
- Deduplication, columns, general improvements
- Copy by reference
- Update doc and convenience stuff
- Formatting
- More trait implementations
- Benchmark updates
- Documentation and performance improvements
- Add docs, utility features, option region
- Remove reserve_items from CopyOnto
- Make ReadItem: CopyOnto<Self>
- Optionally support serde
- Rework structure, tuple support, sizing
- Initial import
