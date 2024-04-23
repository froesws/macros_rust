use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod get_fields;
mod get_fields_values;
mod hello_world;
mod print_fields;

/// Derive the `HelloWorld` trait for a given struct or enum.
#[proc_macro_derive(HelloWorld)]
pub fn hello_world_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let gen = hello_world::impl_hello_world(&input);

    gen
}

/// Derive the `PrintFields` trait for a given struct.
#[proc_macro_derive(PrintFields)]
pub fn print_fields_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let gen = print_fields::impl_print_fields(&input);

    // Return the generated code
    gen.into()
}

#[proc_macro_derive(GetFields)]
pub fn get_fields_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let gen = get_fields::impl_get_fields(&input);

    // Return the generated code
    gen
}

#[proc_macro_derive(GetFieldsValues)]
pub fn get_fields_values_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let gen = get_fields_values::impl_get_fields_values(&input);

    // Return the generated code
    gen
}
