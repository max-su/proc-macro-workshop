extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let struct_data = match input.data {
        Data::Struct(data) => data,
        _ => panic!("not a struct")
    };
    for field in struct_data.fields.iter() {
        println!("{}", field.ident.as_ref().unwrap());
    }

    // let tokens = quote! {
    //     impl #name {
    //         fn builder() -> (#name)Builder {

    //         }
    //     }
    //     struct (#name)Builder {

    //     }

    //     impl (#name)Builder {
    //     }
    // };
    // tokens.into()
    let tokens = quote! {

    };
    tokens.into()
}
