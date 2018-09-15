//! Provides a derive macro that implements `Envconfig` trait.
//! For complete documentation please see [envconfig](https://docs.rs/envconfig).

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

use syn::{Ident, Lit, MetaItem, NestedMetaItem};

#[derive(Debug)]
struct Field {
    name: Ident,
    var_name: String,
}

#[proc_macro_derive(Envconfig, attributes(from, envconfig))]
pub fn derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let struct_name = &ast.ident;

    let field_nodes = fetch_fields_from_ast_body(ast.body, &struct_name.to_string());

    let fields: Vec<Field> = field_nodes.iter().map(|f| parse_field(f.clone())).collect();

    let gen_fields = fields.iter().map(|f| {
        let ident = &f.name;
        let var_name = &f.var_name;
        quote! {
            #ident: ::envconfig::load_var(#var_name)?
        }
    });

    let gen = quote! {
        impl Envconfig for #struct_name {
            fn init() -> Result<Self, ::envconfig::Error> {
                let config = Self {
                    #(#gen_fields,)*
                };
                Ok(config)
            }
        }
    };

    gen.parse().unwrap()
}

fn fetch_fields_from_ast_body(body: syn::Body, name: &str) -> Vec<syn::Field> {
    match body {
        ::syn::Body::Struct(variant_data) => match variant_data {
            ::syn::VariantData::Struct(fields) => fields,
            _ => panic!(
                "Envconfig trait can not be derived from `{}` because it is not a struct.",
                name
            ),
        },
        _ => panic!(
            "Envconfig trait can not be derived from `{}` because it is not a struct.",
            name
        ),
    }
}

fn parse_field(field_node: syn::Field) -> Field {
    let mut from: Option<String> = None;
    // let mut default: Option<::syn::Lit> = None;

    // Get name of the field
    let name = field_node.ident.unwrap();

    // Find `envconfig` attribute on the given field
    let attr = field_node
        .attrs
        .iter()
        .find(|a| a.name() == "envconfig")
        .unwrap_or_else(|| panic!("Field `{}` must have `envconfig` attribute.", name));

    // Unwrap list from `envconfig` attribute.
    let list = match attr.value {
        MetaItem::List(ref _ident, ref list) => list,
        _ => panic!("Envconfig: attribute `envconfig` must be a list"),
    };

    // Iterate over items of `envconfig` attribute.
    // Each item is supposed to have name and value.
    for item in list.iter() {
        let mt = match item {
            NestedMetaItem::MetaItem(mt) => mt,
            _ => panic!(
                "Envconfig: failed to parse `envconfig` attribute for field `{}`",
                name
            ),
        };
        let (ident, value) = match mt {
            MetaItem::NameValue(ident, lit) => (ident, lit),
            _ => panic!(
                "Envconfig: failed to parse `envconfig` attribute for field `{}`",
                name
            ),
        };

        let item_name = format!("{}", ident);

        match item_name.as_str() {
            "from" => match value {
                Lit::Str(s, _style) => {
                    from = Some(s.to_string());
                }
                _ => panic!("Envconfig: value of `from` must be a string"),
            },
            // TODO: handle "default" here as well
            _ => panic!("Envconfig: unknown item on `{}`", item_name),
        }
    }

    let from_value =
        from.unwrap_or_else(|| panic!("attribute `envconfig` must contain `from` item"));

    Field {
        name,
        var_name: from_value,
    }
}
