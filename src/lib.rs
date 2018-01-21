extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use std::iter::repeat;
use syn::{parse, Data, DeriveInput};

use proc_macro::TokenStream;

#[proc_macro_derive(StrToEnum)]
pub fn imp(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(input).unwrap();

    let name = &ast.ident;
    let iname = repeat(name);

    if let Data::Enum(data) = ast.data {
        let fields = data.variants.iter().map(|variant| variant.ident);

        let names = data.variants
            .iter()
            .map(|variant| variant.ident.to_string());

        let token = quote! {
            impl ::std::str::FromStr for #name {
                type Err = ();

                fn from_str(s:&str) -> ::std::result::Result<Self, Self::Err> {
                    match s {
                        #(#names => Ok(#iname::#fields),)*
                        _ => Err(())
                    }
                }
            }
        };

        token.into()
    } else {
        panic!("Only work for enum");
    }
}
