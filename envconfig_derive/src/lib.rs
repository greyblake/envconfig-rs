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
            fn init() -> Result<Self, ::envconfig::Error> {
                let config = Self {
                    #(#field_assigns,)*
                };
                Ok(config)
            }
        }
    }
}

fn gen_field_assign(field: &Field) -> proc_macro2::TokenStream {
    let ident = &field.ident;

    let attr = fetch_envconfig_attr_from_field(field);
    let list = fetch_list_from_attr(field, attr);
    let from_value = fetch_item_from_list(field, &list, "from");

    let field_ty = &field.ty;
    let field_type_name = quote!(#field_ty).to_string();

    if field_type_name.starts_with("Option ") {
        quote! {
            #ident: ::envconfig::load_optional_var(#from_value)?
        }
    } else {
        quote! {
            #ident: ::envconfig::load_var(#from_value)?
        }
    }
}

fn fetch_envconfig_attr_from_field(field: &Field) -> &Attribute {
    field
        .attrs
        .iter()
        .find(|a| {
            let path = &a.path;
            let name = quote!(#path).to_string();
            name == "envconfig"
        }).unwrap_or_else(|| {
            panic!(
                "Can not find attribute `envconfig` on field `{}`",
                field_name(field)
            )
        })
}

fn field_name(field: &Field) -> String {
    let ident = &field.ident;
    quote!(#ident).to_string()
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

fn fetch_item_from_list<'l, 'n>(
    field: &Field,
    list: &'l Punctuated<NestedMeta, Comma>,
    item_name: &'n str,
) -> &'l Lit {
    let item = list
        .iter()
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
        }).find(|name_value| {
            let ident = &name_value.ident;
            quote!(#ident).to_string() == item_name
        }).unwrap_or_else(|| {
            panic!(
                "`envconfig` attribute on field `{}` must contain `{}` item",
                field_name(field),
                item_name
            )
        });

    &item.lit
}
