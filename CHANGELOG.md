# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- Added `ShortGuid::from_bytes_le` as a counterpart to `to_bytes_le`.

## 0.5.0 - 2023-06-24

### Added

- Added a `FromStr` implementation, allowing for `parse::<ShortGuid>("...")`.
- Added `from_slice` to construct from a `&[u8]`.
- Added support for Serde.

## 0.4.0 - 2023-06-24

### Added

- Added more `PartialEq` implementations for `String` and `&&str` comparisons.
- Added `Copy`, `Ord` and `PartialOrd` implementations for `ShortGuid`.

## 0.3.0 - 2023-04-08

### Added

- Added `new_random` when using the `random` crate feature.
- Added `new_from_uuid` to create a `ShortGuid` from an existing UUID.

## 0.2.0 - 2023-04-08

### Added

- Added `PartialEq<T>` for `Vec<u8>` and `&[u8; 16]`. 

### Changed

- `to_bytes_le`, `from_bytes_ref`, `as_uuid` and `is_empty` are now `const`.

### Internal

- Added fuzzing targets.

## 0.1.0 - 2023-04-07

### Added

- 🎉 Initial release.
