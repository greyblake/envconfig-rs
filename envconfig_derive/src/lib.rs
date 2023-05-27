//! Provides a derive macro that implements `Envconfig` trait.
//! For complete documentation please see [envconfig](https://docs.rs/envconfig).

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, DeriveInput, Field, Fields, Ident, Meta, Expr};

#[proc_macro_derive(Envconfig, attributes(envconfig))]
pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let gen = impl_envconfig(&derive_input);
    gen.into()
}

enum Source {
    Environment,
    HashMap,
}

fn impl_envconfig(input: &DeriveInput) -> proc_macro2::TokenStream {
    use syn::Data::*;
    let struct_name = &input.ident;

    let inner_impl = match input.data {
        Struct(ref ds) => match ds.fields {
            Fields::Named(ref fields) => impl_envconfig_for_struct(struct_name, &fields.named),
            _ => panic!("envconfig supports only named fields"),
        },
        _ => panic!("envconfig only supports non-tuple structs"),
    };

    quote!(#inner_impl)
}

fn impl_envconfig_for_struct(
    struct_name: &Ident,
    fields: &Punctuated<Field, Comma>,
) -> proc_macro2::TokenStream {
    let field_assigns_env = fields
        .iter()
        .map(|field| gen_field_assign(field, Source::Environment));
    let field_assigns_hashmap = fields
        .iter()
        .map(|field| gen_field_assign(field, Source::HashMap));

    quote! {
        impl Envconfig for #struct_name {
            fn init_from_env() -> ::std::result::Result<Self, ::envconfig::Error> {
                let config = Self {
                    #(#field_assigns_env,)*
                };
                Ok(config)
            }

            fn init_from_hashmap(hashmap: &::std::collections::HashMap<String, String>) -> ::std::result::Result<Self, ::envconfig::Error> {
                let config = Self {
                    #(#field_assigns_hashmap,)*
                };
                Ok(config)
            }

            fn init() -> ::std::result::Result<Self, ::envconfig::Error> {
                Self::init_from_env()
            }
        }
    }
}

fn gen_field_assign(field: &Field, source: Source) -> proc_macro2::TokenStream {
    let attr = fetch_envconfig_attr_from_attrs(&field.attrs);
    if let Some(attr) = attr {
        // if #[envconfig(...)] is there

        // If nested attribute is present
        let nested_value_opt = find_item_in_attr_meta(&field.ident, &attr, "nested");
        if nested_value_opt.is_some() {
            return gen_field_assign_for_struct_type(field, source);
        }

        let opt_default = find_item_in_attr_meta(&field.ident, &attr, "default");

        let from_opt = find_item_in_attr_meta(&field.ident, &attr, "from");
        let env_var = match from_opt {
            Some(v) => quote! { #v },
            None => field_to_env_var(field),
        };

        gen(field, env_var, opt_default, source)
    } else {
        // if #[envconfig(...)] is not present
        let env_var = field_to_env_var(field);
        gen(field, env_var, None, source)
    }
}

fn field_to_env_var(field: &Field) -> proc_macro2::TokenStream {
    let field_name = field.clone().ident.unwrap().to_string().to_uppercase();
    quote! { #field_name }

}

fn gen(
    field: &Field,
    from: proc_macro2::TokenStream,
    opt_default: Option<Expr>,
    source: Source,
) -> proc_macro2::TokenStream {
    let field_type = &field.ty;
    if to_s(field_type).starts_with("Option ") {
        gen_field_assign_for_optional_type(field, from, opt_default, source)
    } else {
        gen_field_assign_for_non_optional_type(field, from, opt_default, source)
    }
}

fn gen_field_assign_for_struct_type(field: &Field, source: Source) -> proc_macro2::TokenStream {
    let ident = &field.ident;
    match &field.ty {
        syn::Type::Path(path) => match source {
            Source::Environment => quote! {
                #ident: #path :: init_from_env()?
            },
            Source::HashMap => quote! {
                #ident: #path :: init_from_hashmap(hashmap)?
            },
        },
        _ => panic!("Expected field type to be a path: {:?}", ident),
    }
}

fn gen_field_assign_for_optional_type(
    field: &Field,
    from: proc_macro2::TokenStream,
    opt_default: Option<Expr>,
    source: Source,
) -> proc_macro2::TokenStream {
    let field_name = &field.ident;

    if opt_default.is_some() {
        panic!("Optional type on field `{}` with default value does not make sense and therefore is not allowed", to_s(field_name));
    }

    match source {
        Source::Environment => quote! {
            #field_name: ::envconfig::load_optional_var(#from, None)?
        },
        Source::HashMap => quote! {
            #field_name: ::envconfig::load_optional_var(#from, Some(hashmap))?
        },
    }
}

fn gen_field_assign_for_non_optional_type(
    field: &Field,
    from: proc_macro2::TokenStream,
    opt_default: Option<Expr>,
    source: Source,
) -> proc_macro2::TokenStream {
    let field_name = &field.ident;

    if let Some(default) = opt_default {
        match source {
            Source::Environment => quote! {
                #field_name: ::envconfig::load_var_with_default(#from, None, #default)?
            },
            Source::HashMap => quote! {
                #field_name: ::envconfig::load_var_with_default(#from, Some(hashmap), #default)?
            },
        }
    } else {
        match source {
            Source::Environment => quote! {
                #field_name: ::envconfig::load_var(#from, None)?
            },
            Source::HashMap => quote! {
                #field_name: ::envconfig::load_var(#from, Some(hashmap))?
            },
        }
    }
}

fn fetch_envconfig_attr_from_attrs(attrs: &Vec<Attribute>) -> Option<&Attribute> {
    attrs.iter().find(|a| {
        let path = &a.path();
        let name = quote!(#path).to_string();
        name == "envconfig"
    })
}

fn find_item_in_attr_meta<'n>(attr_parent_ident: &Option<Ident>, attr: &Attribute, item_name: &'n str) -> Option<Expr> {
    let nested = attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated).ok()?;
    for meta in nested {
        if meta.path().is_ident(item_name) {
            match meta.require_name_value() {
                Ok(m) => {
                    return Some(m.value.clone());
                },
                Err(_) => panic!(
                    "`envconfig` attribute on field `{}` must contain name/value item",
                    to_s(attr_parent_ident)
                ),
            }
        }
    }

    None
}

fn to_s<T: quote::ToTokens>(node: &T) -> String {
    quote!(#node).to_string()
}
