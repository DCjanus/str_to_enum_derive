extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;


#[proc_macro_derive(StrToEnum)]
pub fn str_to_enum(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let name = &ast.ident;
    if let syn::Body::Enum(body) = ast.body {
        let gen = impl_str_to_enum(name, body);
        gen.parse().unwrap()
    } else {
        panic!("Only work for enum");
    }
}

fn impl_str_to_enum(name: &syn::Ident, body: Vec<syn::Variant>) -> quote::Tokens {
    let content = build_content(name, body);
    quote!(
        impl FromStr for #name{
            type Err = ();

            fn from_str(s:&str)->Result<Self,Self::Err>{
                match s {
                    #content
                    _=>Err(())
                }
            }
        }
    )
}

fn build_content(name: &syn::Ident, body: Vec<syn::Variant>) -> syn::Ident {
    let lines: Vec<String> = body.iter()
        .map(|field| format!("\"{field}\" => Ok({enum_name}::{field}),\n",
                             field = field.ident,
                             enum_name = name))
        .collect();
    syn::Ident::from(lines.join(""))
}

