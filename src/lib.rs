//! This library provides a `call2` function that sits somewhere in-between `syn`'s `parse2` and `ParseBuffer::call`:
//! It lets you conveniently apply a parser function to a `proc-macro2` token stream, for example from a `quote!`.
//!
//! [![Zulip Chat](https://img.shields.io/endpoint?label=chat&url=https%3A%2F%2Fiteration-square-automation.schichler.dev%2F.netlify%2Ffunctions%2Fstream_subscribers_shield%3Fstream%3Dproject%252Fcall2-for-syn)](https://iteration-square.schichler.dev/#narrow/stream/project.2Fcall2-for-syn)

#![doc(html_root_url = "https://docs.rs/call2-for-syn/3.0.4")]
#![warn(clippy::pedantic, missing_docs)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![forbid(unsafe_code)]

use proc_macro2::TokenStream;
use std::{
	error::Error,
	fmt::{self, Debug, Display, Formatter},
	result::Result as stdResult,
};
use syn::parse::{ParseStream, Parser};

//FIXME: Reenable this once there's a Syn 2 version of unquote.
#[cfg(all(never, not(never)))]
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

/// Analogous to [`syn::parse2`] and [`syn::parse::ParseBuffer::call`].
///
/// This function ignores an underlying check whether input was fully parsed.
///
/// # Examples
///
/// ```rust
/// use call2_for_syn::call2_allow_incomplete;
/// use quote::quote;
/// use syn::{Ident, Token};
///
/// # (|| {
/// let (hello, world) = call2_allow_incomplete(quote!(Hello world!), |input| {
///     let hello: Ident = input.parse()?;
///     let world: Ident = input.parse()?;
///     // input.parse::<Token![!]>()?;
///     syn::Result::Ok((hello, world))
/// })?;
///
/// assert_eq!(format!("{}", hello), "Hello");
/// assert_eq!(format!("{}", world), "world");
/// # syn::Result::Ok(())
/// # })().unwrap();
/// ```
///
/// ```rust
/// use call2_for_syn::call2_allow_incomplete;
/// use quote::quote;
/// use syn::{Ident, Token};
///
/// # (|| {
/// let (hello, world) = call2_allow_incomplete(quote!(Hello world!), |input| {
///     let hello: Ident = input.parse()?;
///     let world: Ident = input.parse()?;
///     input.parse::<Token![!]>()?;
///     syn::Result::Ok((hello, world))
/// })?;
///
/// assert_eq!(format!("{}", hello), "Hello");
/// assert_eq!(format!("{}", world), "world");
/// # syn::Result::Ok(())
/// # })().unwrap();
/// ```
///
/// [`syn::parse2`]: https://docs.rs/syn/1.0.14/syn/fn.parse2.html
/// [`syn::parse::ParseBuffer::call`]: https://docs.rs/syn/1.0.14/syn/parse/struct.ParseBuffer.html#method.call
pub fn call2_allow_incomplete<T, P: FnOnce(ParseStream) -> T>(input: TokenStream, parser: P) -> T {
	let mut result = None;
	Parser::parse2(
		|input: ParseStream| {
			result = Some(parser(input));
			Ok(())
		},
		input,
	)
	.ok();
	match result {
		Some(result) => result,
		None => {
			unreachable!()
		}
	}
}

/// Analogous to [`syn::parse2`] and [`syn::parse::ParseBuffer::call`].
///
/// # Errors
///
/// Iff not all of `input` is consumed by `parser`.  
/// `parser`'s result is still returned in the error value, in this case.
///
/// # Examples
///
/// ```rust
/// use call2_for_syn::call2_strict;
/// use quote::quote;
/// use syn::{Ident, Token};
///
/// # (|| {
/// let (hello, world) = call2_strict(quote!(Hello world!), |input| {
///     let hello: Ident = input.parse()?;
///     let world: Ident = input.parse()?;
///     // input.parse::<Token![!]>()?;
///     syn::Result::Ok((hello, world))
/// }).unwrap_err().parsed?;
///
/// assert_eq!(format!("{}", hello), "Hello");
/// assert_eq!(format!("{}", world), "world");
/// # syn::Result::Ok(())
/// # })().unwrap();
/// ```
///
/// ```rust
/// use call2_for_syn::call2_strict;
/// use quote::quote;
/// use syn::{Ident, Token};
///
/// # (|| {
/// let (hello, world) = call2_strict(quote!(Hello world!), |input| {
///     let hello: Ident = input.parse()?;
///     let world: Ident = input.parse()?;
///     input.parse::<Token![!]>()?;
///     syn::Result::Ok((hello, world))
/// })??;
///
/// assert_eq!(format!("{}", hello), "Hello");
/// assert_eq!(format!("{}", world), "world");
/// # Result::<_, Box<dyn std::error::Error>>::Ok(())
/// # })().unwrap();
/// ```
///
/// [`syn::parse2`]: https://docs.rs/syn/1.0.14/syn/fn.parse2.html
/// [`syn::parse::ParseBuffer::call`]: https://docs.rs/syn/1.0.14/syn/parse/struct.ParseBuffer.html#method.call
#[track_caller]
#[allow(clippy::missing_panics_doc)]
pub fn call2_strict<T, P: FnOnce(ParseStream) -> T>(
	input: TokenStream,
	parser: P,
) -> stdResult<T, Incomplete<T>> {
	let mut parsed = None;
	match Parser::parse2(
		|input: ParseStream| {
			parsed = Some(parser(input));
			Ok(())
		},
		input,
	) {
		Ok(()) => Ok(parsed.expect("infallible")),
		Err(syn_error) => Err(Incomplete {
			parsed: parsed.expect("infallible"),
			syn_error,
		}),
	}
}

/// Returned by [`call2_strict`].
///
/// [`call2_strict`]: ./fn.call2_strict.html
pub type Result<T> = stdResult<T, Incomplete<T>>;

/// Signifies that `parser` did not consume all of `input` in a [`call2_strict`] call.
///
/// [`call2_strict`]: ./fn.call2_strict.html
#[derive(Debug, Clone)]
pub struct Incomplete<T> {
	/// The parsed instance.
	pub parsed: T,
	/// The [`Error`](syn::Error) raised because not all input was parsed.
	pub syn_error: syn::Error,
}

impl<T: Debug> Error for Incomplete<T> {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		Some(&self.syn_error)
	}
}

impl<T> Display for Incomplete<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "TokenStream parsed incompletely")
	}
}
