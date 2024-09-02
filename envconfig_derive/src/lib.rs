//! Provides a derive macro that implements `Envconfig` trait.

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, Data::Struct, DeriveInput, Expr, Field,
    Fields, Ident, Lit, Meta, MetaNameValue, Token,
};

/// Custom derive for trait [`envconfig::Envconfig`]
#[proc_macro_derive(Envconfig, attributes(envconfig))]
pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let gen = impl_envconfig(&derive_input);
    gen.into()
}

/// Source type for envconfig variables.
///
/// - `Environment`: Environment variables.
/// - `HashMap`: [`std::collections::HashMap`] for mocking environments in tests.
enum Source {
    Environment,
    HashMap,
}

/// Wrapper for [`impl_envconfig_for_struct`].
///
/// Checks if the provided input is a struct and generates the desired `impl EnvConfig`
///
/// # Panics
/// Panics if `input.data` isn't a struct
fn impl_envconfig(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = &input.ident;

    // Check if derive input is a struct and contains named fields. Panic otherwise
    let named_fields = match input.data {
        Struct(ref ds) => match ds.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => panic!("envconfig supports only named fields"),
        },
        _ => panic!("envconfig only supports non-tuple structs"),
    };

    let inner_impl = impl_envconfig_for_struct(struct_name, &named_fields);

    quote!(#inner_impl)
}

/// Generates the `impl Envconfig` blocks for the provided struct
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

/// Generates the field assignments for the config struct
fn gen_field_assign(field: &Field, source: Source) -> proc_macro2::TokenStream {
    let attr = fetch_envconfig_attr_from_field(field);

    if let Some(attr) = attr {
        // if #[envconfig(...)] is there
        let list = fetch_list_from_attr(field, attr);

        // If nested attribute is present
        let nested_value_opt = find_item_in_list(&list, "nested");
        if nested_value_opt.is_some() {
            return gen_field_assign_for_struct_type(field, source);
        }

        let opt_default = find_item_in_list(&list, "default").unwrap_or(None);

        let from_opt = find_item_in_list(&list, "from");
        let env_var = match from_opt {
            Some(v) => quote! { #v },
            None => field_to_env_var_name(field),
        };

        gen(field, env_var, opt_default, source)
    } else {
        // if #[envconfig(...)] is not present
        // use field name as name of the environment variable
        let env_var = field_to_env_var_name(field);
        gen(field, env_var, None, source)
    }
}

/// Turns the field name into an uppercase [`proc_macro2::TokenStream`]
fn field_to_env_var_name(field: &Field) -> proc_macro2::TokenStream {
    let field_name = field.ident.clone().unwrap().to_string().to_uppercase();
    quote! { #field_name }
}

fn gen(
    field: &Field,
    from: proc_macro2::TokenStream,
    opt_default: Option<&Lit>,
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
    let ident: &Option<Ident> = &field.ident;
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
    opt_default: Option<&Lit>,
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
    opt_default: Option<&Lit>,
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

/// Try to get the `envconfig` attribute from the provided field
fn fetch_envconfig_attr_from_field(field: &Field) -> Option<&Attribute> {
    field.attrs.iter().find(|a| {
        let path = &a.path();
        let name = quote!(#path).to_string();
        name == "envconfig"
    })
}

/// Retrieves the [`syn::atr::MetaList`] for the provided attribute
fn fetch_list_from_attr(field: &Field, attr: &Attribute) -> Vec<Meta> {
    let opt_meta = &attr.meta;

    match opt_meta {
        Meta::List(l) => l
            .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
            .unwrap_or_else(|err| {
                panic!(
                    "{:?} in `envconfig` attribute on field `{}`",
                    err,
                    field_name(field)
                )
            })
            .iter()
            .map(|meta| meta.clone())
            .collect(),
        _ => vec![opt_meta.clone()],
    }
}

/// Tries to find a matching item in the provided list
///
/// # Panics
/// - Multiple items with the same name exist
/// - The item is not a name-value pair or a path
fn find_item_in_list<'l, 'n>(list: &'l Vec<Meta>, item_name: &'n str) -> Option<Option<&'l Lit>> {
    // Find all items with the provided name
    let matching_items = list
        .iter()
        .filter(|token_tree| token_tree.path().is_ident(item_name))
        .collect::<Vec<_>>();

    // Error if multiple matching items are found
    if matching_items.len() > 1 {
        panic!(
            "Found multiple `{}` attributes in `envconfig` attribute",
            item_name
        );
    }

    let matching_result = matching_items.first();

    if let Some(meta) = matching_result {
        return match meta {
            Meta::NameValue(MetaNameValue {
                value: Expr::Lit(value),
                ..
            }) => Some(Some(&value.lit)),
            Meta::Path(_) => Some(None),
            _ => panic!("Expected `{}` to be a name-value pair or a path", item_name),
        };
    }

    None
}

fn field_name(field: &Field) -> String {
    to_s(&field.ident)
}

fn to_s<T: quote::ToTokens>(node: &T) -> String {
    quote!(#node).to_string()
}
