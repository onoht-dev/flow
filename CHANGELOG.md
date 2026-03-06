# Changelog

All notable changes to this project will be documented in this file.

## [0.3.1](https://github.com/onoht-dev/flow/compare/v0.3.0...v0.3.1) - 2026-03-06

### Fixed

- resolve temporary value lifetime issue in git.rs

### Other

- replace git2 with shell commands to reduce dependencies

## [0.3.0](https://github.com/onoht-dev/flow/compare/v0.2.3...v0.3.0) - 2026-03-06

### Added

- *(history)* add search and repo filter options ([#9](https://github.com/onoht-dev/flow/pull/9))

### Other

- add CI workflow for automated testing ([#7](https://github.com/onoht-dev/flow/pull/7))

## [0.2.3](https://github.com/onoht-dev/flow/compare/v0.2.2...v0.2.3) - 2026-03-06

### Added

- implement context history tracking
- initial implementation of Flow

### Fixed

- handle unborn git branches and improve test coverage

### Other

- release v0.2.2 ([#5](https://github.com/onoht-dev/flow/pull/5))
- release v0.2.1 ([#4](https://github.com/onoht-dev/flow/pull/4))
- use correct release-plz action
- fix workflow to trigger on master branch
- improve contributor experience with hooks, docs, and automated releases ([#2](https://github.com/onoht-dev/flow/pull/2))
- bump version to 0.2.0
- Update badge to #000000
- Simplify README with new branding
- Update README with onoht branding

## [0.2.2](https://github.com/onoht-dev/flow/compare/v0.2.1...v0.2.2) - 2026-03-06

### Added

- implement context history tracking
- initial implementation of Flow

### Fixed

- handle unborn git branches and improve test coverage

### Other

- release v0.2.1 ([#4](https://github.com/onoht-dev/flow/pull/4))
- use correct release-plz action
- fix workflow to trigger on master branch
- improve contributor experience with hooks, docs, and automated releases ([#2](https://github.com/onoht-dev/flow/pull/2))
- bump version to 0.2.0
- Update badge to #000000
- Simplify README with new branding
- Update README with onoht branding

## [0.2.1](https://github.com/onoht-dev/flow/compare/v0.2.0...v0.2.1) - 2026-03-06

### Added

- implement context history tracking
- initial implementation of Flow

### Fixed

- handle unborn git branches and improve test coverage

### Other

- use correct release-plz action
- fix workflow to trigger on master branch
- improve contributor experience with hooks, docs, and automated releases ([#2](https://github.com/onoht-dev/flow/pull/2))
- bump version to 0.2.0
- Update badge to #000000
- Simplify README with new branding
- Update README with onoht branding

## [0.2.0] - 2025-03-02

### Features

- Context history tracking
- History command for viewing past contexts

## [0.1.0] - 2025-02-28

### Features

- Core CLI commands (note, status, resume, done)
- Git-aware context detection
- JSON-based storage
- Fast context capture (< 50ms)
