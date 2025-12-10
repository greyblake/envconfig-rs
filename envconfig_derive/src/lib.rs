//! Provides a derive macro that implements `Envconfig` trait.

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, Data::Struct, DeriveInput, Expr, Field,
    Fields, Ident, Lit, Meta, MetaNameValue, Token,
};

/// Custom derive for trait [`envconfig::Envconfig`]
///
/// # Panics
/// - The provided [`TokenStream`] cannot be parsed
/// - The provided input is not a named struct
/// - Invalid configuration in the `envconfig` attributes
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

    let inner_impl = impl_envconfig_for_struct(struct_name, named_fields);

    quote!(#inner_impl)
}

/// Generates the `impl Envconfig` blocks for the provided struct
fn impl_envconfig_for_struct(
    struct_name: &Ident,
    fields: &Punctuated<Field, Comma>,
) -> proc_macro2::TokenStream {
    let field_assigns_env = fields
        .iter()
        .map(|field| gen_field_assign(field, &Source::Environment));
    let field_assigns_hashmap = fields
        .iter()
        .map(|field| gen_field_assign(field, &Source::HashMap));

    quote! {
        impl Envconfig for #struct_name {
            fn init_from_env() -> ::std::result::Result<Self, ::envconfig::Error> {
                let config = Self {
                    #(#field_assigns_env,)*
                };
                ::core::result::Result::Ok(config)
            }

            fn init_from_hashmap(hashmap: &::std::collections::HashMap<String, String>) -> ::std::result::Result<Self, ::envconfig::Error> {
                let config = Self {
                    #(#field_assigns_hashmap,)*
                };
                ::core::result::Result::Ok(config)
            }

            #[deprecated(since="0.10.0", note="Please use `::init_from_env` instead")]
            fn init() -> ::std::result::Result<Self, ::envconfig::Error> {
                Self::init_from_env()
            }
        }
    }
}

/// Generates the field assignments for the config struct
fn gen_field_assign(field: &Field, source: &Source) -> proc_macro2::TokenStream {
    let attr = fetch_envconfig_attr_from_field(field);

    if let Some(attr) = attr {
        // if #[envconfig(...)] is there
        let list = fetch_args_from_attr(field, attr);

        // If nested attribute is present
        let nested_value_opt = find_item_in_list(&list, "nested");
        match nested_value_opt {
            Some(MatchingItem::NoValue) => return gen_field_assign_for_struct_type(field, source),
            Some(MatchingItem::WithValue(_)) => {
                panic!("`nested` attribute must not have a value")
            }
            None => {}
        }

        // Default value for the field
        let opt_default = match find_item_in_list(&list, "default") {
            Some(MatchingItem::WithValue(v)) => Some(v),
            Some(MatchingItem::NoValue) => panic!("`default` attribute must have a value"),
            None => None,
        };

        // Environment variable name
        let from_opt = find_item_in_list(&list, "from");
        let env_var = match from_opt {
            Some(MatchingItem::WithValue(v)) => quote! { #v },
            Some(MatchingItem::NoValue) => panic!("`from` attribute must have a value"),
            None => field_to_env_var_name(field),
        };

        gen(field, &env_var, opt_default, source)
    } else {
        // if #[envconfig(...)] is not present
        // use field name as name of the environment variable
        let env_var = field_to_env_var_name(field);
        gen(field, &env_var, None, source)
    }
}

/// Turns the field name into an uppercase [`proc_macro2::TokenStream`]
///
/// # Panics
/// Panics if the field does not have an identifier
fn field_to_env_var_name(field: &Field) -> proc_macro2::TokenStream {
    let field_name = field.ident.clone().unwrap().to_string().to_uppercase();
    quote! { #field_name }
}

/// Generates the derived field assignment for the provided field
fn gen(
    field: &Field,
    from: &proc_macro2::TokenStream,
    opt_default: Option<&Lit>,
    source: &Source,
) -> proc_macro2::TokenStream {
    let field_type = &field.ty;
    if to_s(field_type).starts_with("Option ") {
        gen_field_assign_for_optional_type(field, from, opt_default, source)
    } else {
        gen_field_assign_for_non_optional_type(field, from, opt_default, source)
    }
}

/// Generates the derived field assignment for a (nested) struct type
///
/// # Panics
/// Panics if the field type is not a path
fn gen_field_assign_for_struct_type(field: &Field, source: &Source) -> proc_macro2::TokenStream {
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
        _ => panic!("Expected field type to be a path: {ident:?}",),
    }
}

/// Generates the derived field assignment for an optional type
///
/// # Panics
/// Panics if the field is an optional type with a default value
fn gen_field_assign_for_optional_type(
    field: &Field,
    from: &proc_macro2::TokenStream,
    opt_default: Option<&Lit>,
    source: &Source,
) -> proc_macro2::TokenStream {
    let field_name = &field.ident;

    assert!(opt_default.is_none(), "Optional type on field `{}` with default value does not make sense and therefore is not allowed", to_s(field_name));

    match source {
        Source::Environment => quote! {
            #field_name: ::envconfig::load_optional_var::<_,::std::collections::hash_map::RandomState>(#from, None)?
        },
        Source::HashMap => quote! {
            #field_name: ::envconfig::load_optional_var::<_,::std::collections::hash_map::RandomState>(#from, Some(hashmap))?
        },
    }
}

/// Generates the derived field assignment for non-optional types
fn gen_field_assign_for_non_optional_type(
    field: &Field,
    from: &proc_macro2::TokenStream,
    opt_default: Option<&Lit>,
    source: &Source,
) -> proc_macro2::TokenStream {
    let field_name = &field.ident;

    if let Some(default) = opt_default {
        match source {
            Source::Environment => quote! {
                #field_name: ::envconfig::load_var_with_default::<_,::std::collections::hash_map::RandomState>(#from, None, #default)?
            },
            Source::HashMap => quote! {
                #field_name: ::envconfig::load_var_with_default::<_,::std::collections::hash_map::RandomState>(#from, Some(hashmap), #default)?
            },
        }
    } else {
        match source {
            Source::Environment => quote! {
                #field_name: ::envconfig::load_var::<_,::std::collections::hash_map::RandomState>(#from, None)?
            },
            Source::HashMap => quote! {
                #field_name: ::envconfig::load_var::<_,::std::collections::hash_map::RandomState>(#from, Some(hashmap))?
            },
        }
    }
}

/// Tries to get the (first) `envconfig` attribute from the provided field
fn fetch_envconfig_attr_from_field(field: &Field) -> Option<&Attribute> {
    field.attrs.iter().find(|a| {
        let path = &a.path();
        let name = quote!(#path).to_string();
        name == "envconfig"
    })
}

/// Fetches the arguments from the provided attribute
///
/// # Panics
/// Panics if the attribute cannot be parsed
fn fetch_args_from_attr(field: &Field, attr: &Attribute) -> Vec<Meta> {
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
            .cloned()
            .collect(),
        _ => vec![opt_meta.clone()],
    }
}

/// Represents the result of a search for an item in a [`Meta`] list
enum MatchingItem<'a> {
    WithValue(&'a Lit),
    NoValue,
}

/// Tries to find the first matching item in the provided list
///
/// # Returns
///
/// - `MatchingItem::WithValue(&Lit)` if a name-value pair is found
/// - `MatchingItem::NoValue` if a path is found
/// - `None` if no matching item is found
///
/// # Panics
///
/// - Multiple items with the same name exist
/// - The item is not a name-value pair or a path
fn find_item_in_list<'l>(list: &'l [Meta], item_name: &str) -> Option<MatchingItem<'l>> {
    // Find all items with the provided name
    let matching_items = list
        .iter()
        .filter(|token_tree| token_tree.path().is_ident(item_name))
        .collect::<Vec<_>>();

    // Check that there is at most one item with the provided name. Error otherwise
    assert!(
        matching_items.len() <= 1,
        "Found multiple `{item_name}` attributes in `envconfig` attribute",
    );

    let matching_result = matching_items.first();

    if let Some(meta) = matching_result {
        return match meta {
            Meta::NameValue(MetaNameValue {
                value: Expr::Lit(value),
                ..
            }) => Some(MatchingItem::WithValue(&value.lit)),
            Meta::Path(_) => Some(MatchingItem::NoValue),
            _ => panic!("Expected `{item_name}` to be a name-value pair or a path"),
        };
    }

    None
}

/// Returns the name of the field as a string
fn field_name(field: &Field) -> String {
    to_s(&field.ident)
}

/// Converts a [`quote::ToTokens`] to a string
fn to_s<T: quote::ToTokens>(node: &T) -> String {
    quote!(#node).to_string()
}
