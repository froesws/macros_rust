use proc_macro::TokenStream;

/// Implement the `PrintFields` trait for a given struct.
pub fn impl_print_fields(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        syn::Data::Struct(data_struct) => &data_struct.fields,
        _ => panic!("PrintFields can only be derived for structs"),
    };

    let mut field_tokens = quote::quote! {};

    for field in fields.iter() {
        let field_name = match &field.ident {
            Some(ident) => ident.to_string(),
            None => panic!("PrintFields can only be derived for named fields"),
        };
        let field_token = quote::quote! {
            println!("Field: {}", #field_name);
        };

        // Append the field token to the field tokens
        field_tokens = quote::quote! {
            #field_tokens
            #field_token
        };
    }

    let gen = quote::quote! {
        impl PrintFields for #name {
            fn print_fields() {
                #field_tokens
            }
        }
    };

    gen.into()
}
