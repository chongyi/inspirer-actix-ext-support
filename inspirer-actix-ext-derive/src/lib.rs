#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

use syn::DeriveInput;

mod service;

#[proc_macro_derive(IntoService)]
pub fn service_derive(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let mut input = syn::parse2::<DeriveInput>(input).unwrap();

    service::expand_service_derive(&mut input).into()
}

#[proc_macro_derive(FromRequest)]
pub fn from_request_service_derive(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let mut input = syn::parse2::<DeriveInput>(input).unwrap();

    service::expand_from_request_service_derive(&mut input).into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
