#![doc(html_root_url = "https://docs.rs/call2-for-syn/1.0.0")]

use {
    core::{cell::RefCell, mem::transmute, ptr::NonNull},
    proc_macro2::TokenStream,
    syn::{
        parse::{Parse, ParseStream, Result},
        parse2,
    },
};

thread_local! {
    #[allow(clippy::type_complexity)]
    static PARSER: RefCell<Option<&'static mut dyn FnMut(ParseStream) -> NonNull<()>>> = RefCell::default();
    static RESULT: RefCell<Option<NonNull<()>>> = RefCell::default();
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
/// assert_eq!(format!("{}", hello), "Hello");
/// assert_eq!(format!("{}", world), "world");
/// # syn::Result::Ok(())
/// # })().unwrap();
/// ```
///
/// [`syn::parse2`]: https://docs.rs/syn/1.0.14/syn/fn.parse2.html
/// [`syn::parse::ParseBuffer::call`]: https://docs.rs/syn/1.0.14/syn/parse/struct.ParseBuffer.html#method.call
pub fn call2<T, P: FnMut(ParseStream) -> T>(input: TokenStream, mut parser: P) -> T {
    struct Dummy;
    impl Parse for Dummy {
        fn parse(input: ParseStream) -> Result<Self> {
            PARSER.with(move |parser_static| {
                let result = parser_static.borrow_mut().take().unwrap()(input);
                RESULT.with(move |result_static| {
                    *result_static.borrow_mut() = Some(result);
                });
                Ok(Self)
            })
        }
    }

    let parser: &mut dyn FnMut(ParseStream) -> T = &mut parser;
    let mut parser = move |input: ParseStream| {
        let result: NonNull<T> = Box::leak(Box::new(parser(input))).into();
        result.cast()
    };
    PARSER.with(|parser_static| {
        *parser_static.borrow_mut() = Some(unsafe {
            //SAFETY: parser is only used during the parse2 call, and the reference is discarded there.
            transmute(&mut parser as &mut dyn FnMut(ParseStream) -> NonNull<()>)
        })
    });
    parse2::<Dummy>(input).unwrap();
    *unsafe {
        //SAFETY: This pointer is created using the same type above, and always valid if present.
        Box::<T>::from_raw(
            RESULT
                .with(|result_static| result_static.borrow_mut().take().unwrap())
                .as_ptr() as *mut T,
        )
    }
}
