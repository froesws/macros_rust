use proc_macro::TokenStream;
use quote::quote;

/// Implement the `HelloWorld` trait for a given struct or enum.
pub fn impl_hello_world(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    quote! {
        impl HelloWorld for #name {
            fn hello_world() {
                println!("Hello, World! My name is {}", stringify!(#name));
            }
        }
    }
    .into()
}
