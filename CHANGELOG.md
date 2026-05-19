# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Updated `ethereum-types` from 0.9.0 to 0.16
- Fixed `to_big_endian()` API usage for `ethereum-types` 0.16 (now returns `[u8; N]` instead of writing to a mutable slice)
- Fixed doctest to use correct crate imports and `U256` construction API
- Translated Chinese comments and documentation to English
