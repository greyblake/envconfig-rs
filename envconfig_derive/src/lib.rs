extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

use syn::{Ident, Lit, MetaItem};

#[derive(Debug)]
struct Field {
    name: Ident,
    var_name: String,
}

#[proc_macro_derive(Envconfig, attributes(from))]
pub fn envconfig(input: TokenStream) -> TokenStream {
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
    let name = field_node.clone().ident.unwrap();

    let var_attr = field_node
        .attrs
        .iter()
        .find(|a| a.name() == "from")
        .expect(&format!("Field `{}` must have from attribute", name));

    let var_name = match var_attr.value {
        MetaItem::NameValue(_, Lit::Str(ref val, _)) => val.to_string(),
        _ => panic!(
            "Envconfig: can not fetch value of var attribute for field `{}`",
            name
        ),
    };

    Field { name, var_name }
}
