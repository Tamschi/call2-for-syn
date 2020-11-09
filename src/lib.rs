#![doc(html_root_url = "https://docs.rs/call2-for-syn/1.0.3")]
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

use proc_macro2::TokenStream;
use std::{
	error::Error,
	fmt::{self, Debug, Display, Formatter},
	result::Result as stdResult,
};
use syn::parse::{ParseStream, Parser};

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}

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
	result.unwrap()
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
#[rustversion::attr(since(1.46), track_caller)]
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
		Ok(()) => Ok(parsed.unwrap()),
		Err(syn_error) => Err(Incomplete {
			parsed: parsed.unwrap(),
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
	pub parsed: T,
	pub syn_error: syn::Error,
}

impl<T: Debug> Error for Incomplete<T> {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		Some(&self.syn_error)
	}
}

impl<T> Display for Incomplete<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "TokenStream parsed incompleteley")
	}
}
