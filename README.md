# call2-for-syn

[![Lib.rs](https://img.shields.io/badge/Lib.rs-*-84f)](https://lib.rs/crates/call2-for-syn)
[![Crates.io](https://img.shields.io/crates/v/call2-for-syn)](https://crates.io/crates/call2-for-syn)
[![Docs.rs](https://docs.rs/call2-for-syn/badge.svg)](https://docs.rs/call2-for-syn)

![Rust 1.54](https://img.shields.io/static/v1?logo=Rust&label=&message=1.54&color=grey)
[![CI](https://github.com/Tamschi/call2-for-syn/workflows/CI/badge.svg?branch=develop)](https://github.com/Tamschi/call2-for-syn/actions?query=workflow%3ACI+branch%3Adevelop)
![Crates.io - License](https://img.shields.io/crates/l/call2-for-syn/2.0.3)

[![GitHub](https://img.shields.io/static/v1?logo=GitHub&label=&message=%20&color=grey)](https://github.com/Tamschi/call2-for-syn)
[![open issues](https://img.shields.io/github/issues-raw/Tamschi/call2-for-syn)](https://github.com/Tamschi/call2-for-syn/issues)
[![open pull requests](https://img.shields.io/github/issues-pr-raw/Tamschi/call2-for-syn)](https://github.com/Tamschi/call2-for-syn/pulls)
[![good first issues](https://img.shields.io/github/issues-raw/Tamschi/call2-for-syn/good%20first%20issue?label=good+first+issues)](https://github.com/Tamschi/call2-for-syn/contribute)

[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/call2-for-syn.svg)](https://web.crev.dev/rust-reviews/crate/call2-for-syn/)
[![Zulip Chat](https://img.shields.io/endpoint?label=chat&url=https%3A%2F%2Fiteration-square-automation.schichler.dev%2F.netlify%2Ffunctions%2Fstream_subscribers_shield%3Fstream%3Dproject%252Fcall2-for-syn)](https://iteration-square.schichler.dev/#narrow/stream/project.2Fcall2-for-syn)

This library provides a `call2` function that sits somewhere in-between `syn`'s `parse2` and `ParseBuffer::call`: It lets you conveniently apply a parser function to a `proc-macro2` token stream, for example from a `quote!`.

## Installation

Please use [cargo-edit](https://crates.io/crates/cargo-edit) to always add the latest version of this library:

```cmd
cargo add call2-for-syn
```

## Example

```rust
use call2_for_syn::{call2_allow_incomplete, call2_strict};
use quote::quote;
use syn::{ext::IdentExt as _, Ident, Token};
use unquote::unquote;

let (now, is) = call2_strict(
    quote!(Now is...),
    |input| {
        let now: Ident = input.parse()?;
        let is: Ident = input.parse()?;
        // input.parse::<Token![!]>()?;
        syn::Result::Ok((now, is))
    },
).unwrap_err().parsed.unwrap(); // Use ? here in a real program.

assert_eq!(format!("{}", now), "Now");
assert_eq!(format!("{}", is), "is");

let (a, good, time) = call2_strict(
    quote!(...a good time...),
    |input| {
        let a: Ident;
        let good: Ident;
        let time: Ident;
        unquote!(input, ... #a #good #time ...);
        syn::Result::Ok((a, good, time))
    },
).unwrap().unwrap(); // Use ? here in a real program.

assert_eq!(format!("{}", a), "a");
assert_eq!(format!("{}", good), "good");
assert_eq!(format!("{}", time), "time");

let (r#for, parsing) = call2_allow_incomplete(
    quote!(for parsing!),
    |input| {
        let r#for = input.call(Ident::parse_any)?;
        let parsing: Ident = input.parse()?;
        // input.parse::<Token![!]>()?;
        syn::Result::Ok((r#for, parsing))
    },
).unwrap(); // Use ? here in a real program.

assert_eq!(format!("{}", r#for), "for");
assert_eq!(format!("{}", parsing), "parsing");
```

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING](CONTRIBUTING.md) for more information.

## [Code of Conduct](CODE_OF_CONDUCT.md)

## [Changelog](CHANGELOG.md)

## Versioning

`call2-for-syn` strictly follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with the following exceptions:

- The minor version will not reset to 0 on major version changes (except for v1).  
Consider it the global feature level.
- The patch version will not reset to 0 on major or minor version changes (except for v0.1 and v1).  
Consider it the global patch level.

This includes the Rust version requirement specified above.  
Earlier Rust versions may be compatible, but this can change with minor or patch releases.

Which versions are affected by features and patches can be determined from the respective headings in [CHANGELOG.md](CHANGELOG.md).

Note that dependencies of this crate may have a more lenient MSRV policy!
Please use `cargo +nightly update -Z minimal-versions` in your automation if you don't generate Cargo.lock manually (or as necessary) and require support for a compiler older than current stable.
