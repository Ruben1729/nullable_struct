//! This crate provides a derive macro that makes it easy to create nullable versions of structs.
//!
//! The `Nullable` derive macro wraps each field of a struct in an `Option<T>` type, allowing any field to be set to `None`.
//!
//! It also generates helper methods for getting, setting, and initializing these optional fields.
//!
//! # Example
//!
//! ```rust
//! use your_crate_name::Nullable;
//!
//! #[derive(Nullable)]
//! struct MyStruct {
//!     field1: i32,
//!     field2: String,
//! }
//!
//! let s = NullableTestStruct::new(42, "hello".to_string());
//! assert_eq!(s.field1(), 42);
//! assert_eq!(s.field2(), "hello".to_string());
//!
//! assert_eq!(instance.field1(), 42);
//! assert_eq!(instance.field2(), "Hello, World!".to_string());
//! ```
//!

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, DeriveInput, Fields, Ident};

/// Derive macro to make a struct "nullable" by wrapping all of its fields in `Option<T>` and
/// generating convenient getter and setter methods for these fields.
///
/// Each field's type becomes `Option<T>`, and three kinds of methods are generated for each field:
///
/// - `field(&self) -> T`: A getter that returns the value or the default if the field is `None`.
/// - `get_field(&self) -> Option<&T>`: A getter that returns the `Option<&T>`.
/// - `set_field(&mut self, T)`: A setter that sets the field to `Some(T)`.
///
/// The generated struct also has a `Default` implementation that initializes all fields to `None`.
#[proc_macro_derive(Nullable)]
pub fn nullable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let nullable_name = Ident::new(&format!("Nullable{}", name), name.span());

    // Used to store tokens for each field and their initializations
    if let syn::Data::Struct(data_struct) = input.data {

        let generated_setters_and_getters = generate_setter_and_getter_functions(&data_struct.fields);
        let mut field_tokens = Vec::new();
        let mut field_def_inis = Vec::new();
        let mut field_none_inis = Vec::new();

        let mut field_params = Vec::new();
        let mut identifiers = Vec::new();

        for field in data_struct.fields.iter() {
            let ident = &field.ident;
            let ty = &field.ty;

            field_tokens.push(quote! {
                #ident: std::option::Option<#ty>,
            });

            field_none_inis.push(quote! {
                #ident: None,
            });

            field_def_inis.push(quote! {
                #ident: Some(#ty::default()),
            });

            field_params.push(quote! {
                #ident: #ty,
            });

            identifiers.push(quote! {
                #ident: Some(#ident),
            });
        }

        let expanded = quote! {
            pub struct #nullable_name {
                #(#field_tokens)*
            }

            impl #nullable_name {
                pub fn new(#(#field_params)*) -> Self {
                    Self {
                        #(#identifiers)*
                    }
                }

                pub fn new_default() -> Self {
                    Self {
                        #(#field_def_inis)*
                    }
                }

                #(#generated_setters_and_getters)*
            }

            impl Default for #nullable_name {
                fn default() -> Self {
                    Self {
                        #(#field_none_inis)*
                    }
                }
            }
        };

        return TokenStream::from(expanded);
    }

    panic!("Only structs are nullable.");
}

fn generate_setter_and_getter_functions(fields: &Fields) -> Vec<proc_macro2::TokenStream> {
    fields.iter().map(
        |field| {
            let ty = &field.ty;
            let ident = field.ident.as_ref().expect("No identifier found for type");
            let setter_name = Ident::new(
                &format!("set_{}", ident.to_string().to_lowercase()),
                ident.span(),
            );
            let getter_name = Ident::new(
                &format!("get_{}", ident.to_string().to_lowercase()),
                ident.span(),
            );

            quote! {
                pub fn #ident(&self) -> #ty {
                    self.#ident.clone().unwrap_or_default()
                }

                pub fn #getter_name(&self) -> Option<&#ty> {
                    self.#ident.as_ref()
                }

                pub fn #setter_name(&mut self, value: #ty) {
                    self.#ident = Some(value)
                }
            }
        }).collect()
}
