use proc_macro2::{TokenStream};
use proc_macro2::Ident;

pub fn expand_service_derive(
    input: &mut syn::DeriveInput,
) -> TokenStream {
    let ident = input.ident.clone();
    let (key, field, target) = match &input.data {
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

                    (key, field, target)
                }
                _ => panic!()
            }
        }
        _ => panic!()
    };

    quote! {
        impl IntoService<(#(#target),*,)> for #ident {
            fn init(deps: (#(#target),*,)) -> Self {
                #ident {
                    #(#field: deps.#key,)*
                }
            }
        }
    }
}

pub fn expand_from_request_service_derive(input: &mut syn::DeriveInput) -> TokenStream {
    let ident = input.ident.clone();
    quote! {
        impl actix_web::FromRequest for #ident {
            type Error = actix_web::Error;
            type Future = futures::future::Ready<Result<Self, actix_web::Error>>;
            type Config = ();

            fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
                futures::future::ok(#ident::make(req).unwrap())
            }
        }
    }
}