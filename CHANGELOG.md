# call2-for-syn Changelog

## next

TODO: Date

* Reworked this crate's API to be considerably more useful and not quite as trivial

  Migrate from `call2` to `call2_strict` if you expect to consume all input. Otherwise, migrate to `call2_allow_incomplete`.

* Improved README

## ..3

2020-11-09

* Properly simplified implementation

  This isn't really justified in being a crate anymore (see the new v1 code being one line), but it exists, so I'll maintain it.

## ..2

2020-11-08

* Fixes:
  * Only apply `#[track_caller]` on Rust 1.46 or later

## ..1

2020-11-08

* Fixes:
  * Documented [panic](https://github.com/Tamschi/call2-for-syn/issues/1)
* Greatly simplified implementation
* Added meta data

## 1.0.0

2020-05-11

Initial publication
