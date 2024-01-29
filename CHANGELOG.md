# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com),
and this project adheres to [Semantic Versioning](https://semver.org).

<!--
Types of changes:
- `Added` for new features;
- `Changed` for changes in existing functionality;
- `Deprecated` for soon-to-be removed features;
- `Removed` for now removed features;
- `Fixed` for any bug fixes;
- `Security` in case of vulnerabilities.
-->

<!-- next-header -->
## [Unreleased]

## [0.3.1] - 2024-01-29

### Changed

- Removed unnecessary `Copy` trait bound in `horner_array`.

## [0.3.0] - 2024-01-29

### Added

- Functions `horner` and `horner_array`.

## [0.2.0] - 2024-01-29

### Added

- Macro `estrin!`.
- Macro `estrin_fma!`.

## [0.1.2] - 2024-01-26

### Added

- Macro `horner_fma!`.

## [0.1.1] - 2024-01-26

### Fixed

- Recursive macro call, using `$crate::`.

## [0.1.0] - 2024-01-26

### Added

- Macro `horner!`.

<!-- next-url -->
[Unreleased]: https://github.com/FedericoStra/polyeval/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/FedericoStra/polyeval/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/FedericoStra/polyeval/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/FedericoStra/polyeval/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/FedericoStra/polyeval/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/FedericoStra/polyeval/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/FedericoStra/polyeval/releases/tag/v0.1.0
