# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- feat: support redacting with custom strings (#222)

---

## [0.2.0] - 2024-10-30

### Changed

- Breaking: de-anchor veil imports, to allow for reexporting.

This might cause issues in some very rare cases, but generally shouldn't require any changes.

---

## [0.1.7] - 2023-11-20

### Changed

- Fix redaction of options (#75) thanks to @nick-schafhauser

---

## [0.1.6] - 2023-05-04

### Changed

- Updated to syn v2


[Unreleased]: https://github.com/primait/veil/compare/0.2.0...HEAD
[0.2.0]: https://github.com/primait/veil/compare/0.1.7...0.2.0
[0.1.7]: https://github.com/primait/veil/compare/0.1.6...0.1.7
[0.1.6]: https://github.com/primait/veil/compare/0.1.5...0.1.6
