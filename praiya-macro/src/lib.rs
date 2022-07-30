use proc_macro::{self, TokenStream};
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, FieldsNamed, Type};

#[proc_macro_derive(PraiyaParamsBuilder)]
pub fn builder(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = parse_macro_input!(input);

    let mut fields = vec![];
    match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                for f in named {
                    let ident = f.ident;
                    match f.ty {
                        Type::Path(syn::TypePath {
                            path: syn::Path { segments, .. },
                            ..
                        }) if segments.iter().any(|p| p.ident == "Vec") => {
                            fields.push(quote! {
                                pub fn #ident<I: IntoIterator<Item = &'req str>>(&mut self, #ident: I) -> &mut Self {
                                    for item in #ident {
                                        self.qs.append_pair(&format!("{}[]", stringify!(#ident)), &item);
                                    }
                                    self
                                }                    
                            
                            });
                        }
                        Type::Path(syn::TypePath {
                            path: syn::Path { segments, .. },
                            ..
                        }) if segments.iter().any(|p| p.ident == "String") => {
                            fields.push(quote! {
                                pub fn #ident(&mut self, #ident: &'req str) -> &mut Self {
                                    self.qs.append_pair(stringify!(#ident), &#ident);

                                    self
                                }

                            });
                        }
                        Type::Path(syn::TypePath {
                            path: syn::Path { segments, .. },
                            ..
                        }) if segments.iter().any(|p| p.ident == "chrono") => {
                            fields.push(quote! {
                                pub fn #ident(&mut self, #ident: &'req chrono::DateTime<chrono::Utc>) -> &mut Self {
                                    self.qs.append_pair(stringify!(#ident), &#ident.to_rfc3339());

                                    self
                                }                    
                            
                            });
                        }
                        x => {
                            fields.push(quote! {
                                pub fn #ident(&mut self, #ident: &'req #x) -> &mut Self {
                                    self.qs.append_pair(stringify!(#ident), &format!("{}", &#ident));

                                    self
                                }                    
                            
                            });
                        }
                    }
                }
            }
            _ => (),
        },
        _ => (),
    };

    let mut source_method_doclink = String::new();
    for attr in attrs {
        if attr.path.segments.iter().all(|p| p.ident == "doc") {
            let mut iter = attr.tokens.into_iter();
            iter.next();

            source_method_doclink = format!("{}", iter.next().unwrap());
        }
    }

    let name = Ident::new(&format!("{}Params", ident), Span::call_site());
    let builder = Ident::new(&format!("{}ParamsBuilder", ident), Span::call_site());

    let param_docstring = format!(
        "Options built using [{}] for {}",
        builder,
        source_method_doclink.replace("\"", "")
    );
    let output = quote! {
        #[derive(Default, Serialize)]
        #[doc = #param_docstring]
        pub struct #name {
            pub(crate) qs: String,
        }

        pub struct #builder<'req> {
            qs: url::form_urlencoded::Serializer<'req, String>,
        }

        impl<'req> #builder<'req> {
            fn new() -> Self {
                Self {
                    qs: url::form_urlencoded::Serializer::new(String::new())
                }
            }

            #(#fields)*
        }

        impl<'req> crate::praiya::ParamsBuilder<#name> for #builder<'req> {
            fn build(&mut self) -> #name {
                #name {
                    qs: self.qs.finish(),
                }
            }
        }

        impl crate::praiya::BaseOption for #name {
            fn build_paginated_query_string(&self, pagination: std::sync::Arc<dyn PaginationQueryComponent + Sync + Send>) -> String {
                let mut query = url::form_urlencoded::Serializer::new(self.qs.clone());
                pagination.append_paginated_query_string(&mut query);
                query.finish()
            }
        }
    };

    output.into()
}
