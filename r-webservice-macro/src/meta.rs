use std::collections::HashMap;

use quote::quote;
use proc_macro2::{TokenStream, TokenTree::{Group, Ident, Punct}};
use syn::{Data, DataStruct, DeriveInput, Attribute, Fields, FieldsNamed};

pub fn derive_db_meta_inner(input: &DeriveInput) -> syn::Result<TokenStream> {
    let attrs = &input.attrs;
    let data  = &input.data;
    let name  = &input.ident;
    let attributes = derive_attribute_helper(&attrs, "meta");

    match data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => {
            if let (Some(table_name), Some(primary_key)) 
                = (attributes.get("table_name"), attributes.get("primary_key")) {
                let mut column_str = quote!(
                    let mut columns = ::std::collections::HashMap::new();
                ).to_string();
                for field in named {
                    let ident_attr  = derive_attribute_helper(&field.attrs, "meta");
                    if let Some(ident_meta) = &field.ident {
                        let ident_meta = ident_meta.to_string();
                        if let Some(type_meta) = &ident_attr.get("Type") {
                            column_str += &quote!(
                                columns.insert(#ident_meta.to_string(), #type_meta);
                            ).to_string();
                        } else {
                            return Err(syn::Error::new_spanned(input, "Type are required"));
                        }
                    } else {
                        return Err(syn::Error::new_spanned(input, "Ident are required"));
                    }
                }
                let columns: TokenStream = column_str.parse().unwrap();
                Ok(
                    quote!(
                        impl crate::module::ModuleDBMeta for #name {
                            fn config() -> crate::module::ModuleDBConfig {
                                #columns
                                crate::module::ModuleDBConfig::factory(
                                    #table_name.to_string(),
                                    #primary_key.to_string(),
                                    columns
                                ).unwrap()
                            }
                        }
                    )
                )
            } else {
                Err(syn::Error::new_spanned(input, "table_name are required"))
            }
        },

        _ => Err(syn::Error::new_spanned(input, "Not supported")),
    }
}

fn derive_attribute_helper(attrs: &[Attribute], path: &str) -> HashMap<String, TokenStream> {
    let mut options = HashMap::new();
    for attr in attrs {
        if let Some(segment) = attr.path.segments.first() {
            if &segment.ident.to_string() == path {
                let tokens = attr.tokens.clone().into_iter();
                for token in tokens {
                    if let Group(group) = token {
                        let mut ident   = None;
                        let mut literal = None;

                        let mut iter = group.stream().into_iter();
                        // key
                        while let Some(t) = iter.next() {
                            if let Ident(i) = t {
                                ident = Some(i);
                                break;
                            }
                        }
                        // =
                        while let Some(t) = iter.next() {
                            if let Punct(_) = t {
                                let mut l = "".to_string();
                                while let Some(a) = iter.next() {
                                    l = l + &a.to_string()
                                }
                                literal = Some(l);
                            }
                        }

                        if let Some(ident) = ident {
                            if let Some(literal) = literal {
                                options.insert(ident.to_string(), literal.parse().unwrap());
                            }
                        }
                    }
                }
            }
        }
    }
    options
}