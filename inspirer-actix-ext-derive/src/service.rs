use proc_macro2::TokenStream;

pub fn expand_service_derive(
    input: &mut syn::DeriveInput,
) -> TokenStream {
    let ident = input.ident.clone();
    let result = match &input.data {
        syn::Data::Struct(data_struct) => {
            match &data_struct.fields {
                syn::Fields::Named(fields_named) => {
                    let (mut key, mut field, mut target) = (vec![], vec![], vec![]);
                    for (offset, v) in fields_named.named.iter().enumerate() {
                        key.push(syn::LitInt::new(&format!("{}", offset), proc_macro2::Span::call_site()));
                        field.push(v.ident.clone().unwrap());
                        target.push(match &v.ty {
                            syn::Type::Path(path) => {
                                path.path.segments.first().unwrap().ident.clone()
                            }
                            _ => panic!()
                        })
                    }

                    let block = quote!{
                        #ident {
                            #(#field: deps.#key,)*
                        }
                    };

                    Some((target, block))
                },
                syn::Fields::Unnamed(fields_unnamed) => {
                    let (mut key, mut target) = (vec![], vec![]);
                    for (offset, v) in fields_unnamed.unnamed.iter().enumerate() {
                        key.push(syn::LitInt::new(&format!("{}", offset), proc_macro2::Span::call_site()));
                        target.push(match &v.ty {
                            syn::Type::Path(path) => {
                                path.path.segments.first().unwrap().ident.clone()
                            }
                            _ => panic!()
                        })
                    }

                    let block = quote!{
                        #ident (#(deps.#key),*)
                    };

                    Some((target, block))
                }
                _ => None
            }
        },
        _ => panic!()
    };

    match result {
        Some((target, block)) => {
            quote! {
                impl IntoService<(#(#target),*,)> for #ident {
                    fn init(deps: (#(#target),*,)) -> Self {
                        #block
                    }
                }
            }
        },
        None => {
            quote! {
                impl IntoService<()> for #ident {
                    fn init(deps: ()) -> Self {
                        #ident
                    }
                }
            }
        }
    }


}

pub fn expand_from_request_service_derive(input: &mut syn::DeriveInput) -> TokenStream {
    let ident = input.ident.clone();
    quote! {
        impl actix_web::FromRequest for #ident {
            type Error = inspirer_actix_ext::error::Error;
            type Future = futures::future::Ready<Result<Self, inspirer_actix_ext::error::Error>>;
            type Config = ();

            fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
                match #ident::make(req) {
                    Ok(result) => futures::future::ok(result),
                    Err(err) => futures::future::err(err),
                }

            }
        }
    }
}