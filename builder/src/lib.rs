extern crate proc_macro;

use proc_macro2;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Data, Ident, Fields};

// Creates the struct definition fields
fn builder_definition_fields(data: &Data) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let field_names = fields.named.iter().map(|f| &f.ident);
                    let field_types = fields.named.iter().map(|f| &f.ty);
                    quote! {
                        #(
                            #field_names: Option<#field_types>,
                        )*
                    }
                }, Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

// Creates the initial fields of the builder struct set to None
fn new_builder_fields(data: &Data) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let field_names = fields.named.iter().map(|f| &f.ident);
                    quote! {
                        #(
                            #field_names: None,
                        )*
                    }
                }, Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn builder_setter_methods(data: &Data) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let field_names = fields.named.iter().map(|f| &f.ident);
                    let field_types = fields.named.iter().map(|f| &f.ty);
                    quote! {
                        #(
                            pub fn #field_names(&mut self, #field_names: #field_types) -> &mut Self {
                                self.#field_names = Some(#field_names);
                                self
                            }
                        )*
                    }
                }, Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn builder_method(data: &Data, name: &Ident) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let field_names = fields.named.iter().map(|f| &f.ident);
                    let field_names_0 = field_names.clone();
                    quote! {
                        pub fn build(&self) -> Result<#name, Box<dyn Error>> {
                            #(
                            if (self.#field_names_0.is_none()) {
                                return Err("One value is not initialized".into());
                            }
                            )*
                            Ok(#name {
                                #(
                                    #field_names: self.#field_names.clone().unwrap(),
                                )*
                            })
                        }
                    }
                }, Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }

}

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation for #name
    let name = input.ident;
    // Used in the quasi-quotation for #builder_name
    let builder_name = format_ident!("{}Builder", name);

    let new_builder_fields = new_builder_fields(&input.data);
    let builder_definition_fields = builder_definition_fields(&input.data);
    let setter_methods = builder_setter_methods(&input.data);
    let builder_method = builder_method(&input.data, &name);

    let tokens = quote! {
        use std::error::Error;
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #new_builder_fields
                }
            }
        }
        pub struct #builder_name {
            #builder_definition_fields
        }
        impl #builder_name {
            #setter_methods
            #builder_method
        }
    };
    eprintln!("TOKENS: {}", tokens.clone());
    tokens.into()
}
