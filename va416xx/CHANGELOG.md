Change Log
=======

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [unreleased]

## [v0.2.0] 2024-06-25

- Re-Generated PAC with `svd2rust` v0.33.3
- Improve documentation for generation of PAC.

## [v0.1.1]

- Clippy issue fixed by regenerating PAC with patched `svd2rust`:
  https://github.com/rust-embedded/svd2rust/pull/558

## [v0.1.0]

- Clippy currently complains about unsound code which should still work.
  Related issue: https://github.com/rust-embedded/svd2rust/issues/557
  Clippy is disabled in CI/CD for now.
- Initial release
