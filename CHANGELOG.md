# `branchy` changelog

## 0.2.0 (2019-12-31)
* `Error` fields are now public and `ErrorKind` enum is exported.
* Relaxed `Copy` trait bound to `Clone` for values of both non-terminal and terminal symbols (`NonterminalValue` and `TerminalValue` traits).
* `Symbol` now derives `PartialEq`, `Rule` now derives `Debug`, `Clone` and `PartialEq`.
* Added unit-tests for `expand_input()` as well as 2 integration tests.
* Added some documentation to all exported types.

## 0.1.1 (2019-12-29)
* Fixed invalid links in the documentation.

## 0.1.0 (2019-12-29)
* Inital release.
