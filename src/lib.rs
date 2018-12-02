#![recursion_limit = "128"]
extern crate proc_macro;

use std::hash::{Hash, Hasher};

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(StrToEnum)]
pub fn str_to_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = syn::parse::<DeriveInput>(input).expect("Failed to parse derive input");

    let enum_name = derive_input.ident.clone();
    let data_enum = match derive_input.data {
        syn::Data::Enum(x) => x,
        _ => panic!("Only work for enum"),
    };
    let hash_to_variant = data_enum.variants.iter().map(|x| {
        let variant_name = x.ident.to_string();

        let mut hasher = hasher().0;
        variant_name.hash(&mut hasher);
        let key = hasher.finish();

        quote!(#key => Ok(#enum_name::#x))
    });

    let new_hasher_token_stream = hasher().1;
    let output = quote! {
        impl std::str::FromStr for #enum_name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let mut hasher = #new_hasher_token_stream;
                std::hash::Hash::hash(&s, &mut hasher);

                match std::hash::Hasher::finish(&hasher) {
                    #(#hash_to_variant,)*
                    _ => Err(())
                }
            }
        }
    };
    output.into()
}

#[cfg(not(feature = "fxhasher"))]
fn hasher() -> (impl Hasher, TokenStream) {
    (
        std::collections::hash_map::DefaultHasher::default(),
        quote!(std::collections::hash_map::DefaultHasher::default()),
    )
}

#[cfg(feature = "fxhasher")]
fn hasher() -> (impl Hasher, TokenStream) {
    (
        fxhash::FxHasher::default(),
        quote!(fxhash::FxHasher::default()),
    )
}
