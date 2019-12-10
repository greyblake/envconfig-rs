//! Provides a derive macro that implements `Envconfig` trait.
//! For complete documentation please see [envconfig](https://docs.rs/envconfig).

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, DeriveInput, Field, Fields, Ident, Lit, Meta, NestedMeta};

#[proc_macro_derive(Envconfig, attributes(envconfig))]
pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let gen = impl_envconfig(&derive_input);
    gen.into()
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
    let field_assigns = fields.iter().map(gen_field_assign);

    quote! {
        impl Envconfig for #struct_name {
            fn init() -> ::std::result::Result<Self, ::envconfig::Error> {
                let config = Self {
                    #(#field_assigns,)*
                };
                Ok(config)
            }
        }
    }
}

fn gen_field_assign(field: &Field) -> proc_macro2::TokenStream {
    let attr = fetch_envconfig_attr_from_field(field);

    if let Some(attr) = attr {
        let list = fetch_list_from_attr(field, attr);
        let from_value = find_item_in_list_or_panic(field, &list, "from");
        let opt_default = find_item_in_list(field, &list, "default");
    
        let field_type = &field.ty;
    
        if to_s(field_type).starts_with("Option ") {
            gen_field_assign_for_optional_type(field, from_value, opt_default)
        } else {
            gen_field_assign_for_non_optional_type(field, from_value, opt_default)
        }
    } else {
        gen_field_assign_for_struct_type(field)
    }

}

fn gen_field_assign_for_struct_type(
    field: &Field,
) -> proc_macro2::TokenStream {
    let ident = &field.ident;
    match &field.ty {
        syn::Type::Path(path) => {
            quote! {
                #ident: #path :: init()?
            }
        },
        _ => {
            panic!(
                "AAA",
            )
        }
    }
}

fn gen_field_assign_for_optional_type(
    field: &Field,
    from: &Lit,
    opt_default: Option<&Lit>,
) -> proc_macro2::TokenStream {
    let ident = &field.ident;

    if opt_default.is_some() {
        panic!("Optional type on field `{}` with default value does not make sense and therefore is not allowed", to_s(ident));
    } else {
        quote! {
            #ident: ::envconfig::load_optional_var(#from)?
        }
    }
}

fn gen_field_assign_for_non_optional_type(
    field: &Field,
    from: &Lit,
    opt_default: Option<&Lit>,
) -> proc_macro2::TokenStream {
    let ident = &field.ident;

    if let Some(default) = opt_default {
        quote! {
            #ident: ::envconfig::load_var_with_default(#from, #default)?
        }
    } else {
        quote! {
            #ident: ::envconfig::load_var(#from)?
        }
    }
}

fn fetch_envconfig_attr_from_field(field: &Field) -> Option<&Attribute> {
    field
        .attrs
        .iter()
        .find(|a| {
            let path = &a.path;
            let name = quote!(#path).to_string();
            name == "envconfig"
        })
}

fn fetch_list_from_attr(field: &Field, attr: &Attribute) -> Punctuated<NestedMeta, Comma> {
    let opt_meta = attr.interpret_meta().unwrap_or_else(|| {
        panic!(
            "Can not interpret meta of `envconfig` attribute on field `{}`",
            field_name(field)
        )
    });

    match opt_meta {
        Meta::List(l) => l.nested,
        _ => panic!(
            "`envconfig` attribute on field `{}` must contain a list",
            field_name(field)
        ),
    }
}

fn find_item_in_list_or_panic<'l, 'n>(
    field: &Field,
    list: &'l Punctuated<NestedMeta, Comma>,
    item_name: &'n str,
) -> &'l Lit {
    find_item_in_list(field, list, item_name).unwrap_or_else(|| {
        panic!(
            "`envconfig` attribute on field `{}` must contain `{}` item",
            field_name(field),
            item_name
        )
    })
}

fn find_item_in_list<'l, 'n>(
    field: &Field,
    list: &'l Punctuated<NestedMeta, Comma>,
    item_name: &'n str,
) -> Option<&'l Lit> {
    list.iter()
        .map(|item| match item {
            NestedMeta::Meta(meta) => match meta {
                Meta::NameValue(name_value) => name_value,
                _ => panic!(
                    "`envconfig` attribute on field `{}` must contain name/value item",
                    field_name(field)
                ),
            },
            _ => panic!(
                "Failed to process `envconfig` attribute on field `{}`",
                field_name(field)
            ),
        })
        .find(|name_value| {
            let ident = &name_value.ident;
            let name = quote!(#ident).to_string();
            name == item_name
        })
        .map(|item| &item.lit)
}

fn field_name(field: &Field) -> String {
    to_s(&field.ident)
}

fn to_s<T: quote::ToTokens>(node: &T) -> String {
    quote!(#node).to_string()
}
