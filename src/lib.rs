#![doc(html_root_url = "https://docs.rs/call2-for-syn/1.0.0")]
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

use proc_macro2::TokenStream;
use syn::parse::{ParseStream, Parser};

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}

/// Analogous to [`syn::parse2`] and [`syn::parse::ParseBuffer::call`].
///
/// # Example
///
/// ```rust
/// use {
///     call2_for_syn::call2,
///     quote::quote,
///     syn::{Ident, Token},
/// };
///
/// # (|| {
/// let (hello, world) = call2::<syn::Result<_>, _>(quote!(Hello world!), |input| {
///     let hello: Ident = input.parse()?;
///     let world: Ident = input.parse()?;
///     input.parse::<Token![!]>()?;
///     Ok((hello, world))
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
pub fn call2<T, P: FnOnce(ParseStream) -> T>(input: TokenStream, parser: P) -> T {
	let mut result: Option<T> = None;
	Parser::parse2(
		|input: ParseStream| {
			result = Some(parser(input));
			Ok(())
		},
		input,
	)
	.unwrap();
	result.unwrap()
}
