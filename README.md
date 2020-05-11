# call2-for-syn

[![Latest Version](https://img.shields.io/crates/v/call2-for-syn.svg)](https://crates.io/crates/call2-for-syn)
[![docs.rs](https://docs.rs/call2-for-syn/badge.svg?version=1.0.0)](https://docs.rs/call2-for-syn/1.0.0/call2_for_syn/)

This library provides a `call2` function that sits somewhere in-between `syn`'s `parse2` and `ParseBuffer::call`: It lets you conveniently apply a parser function to a `proc-macro2` token stream, for example from a `quote!`.

## Example

```rust
use {
    call2_for_syn::call2,
    quote::quote,
    syn::{Ident, Token},
};

let (hello, world) = call2::<syn::Result<_>, _>(
    quote!(Hello world!),
    |input| {
        let hello: Ident = input.parse()?;
        let world: Ident = input.parse()?;
        input.parse::<Token![!]>()?;
        Ok((hello, world))
    },
)?;
assert_eq!(format!("{}", hello), "Hello");
assert_eq!(format!("{}", world), "world");
```
