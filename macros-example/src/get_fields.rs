use proc_macro::TokenStream;

/// Implement the `GetFields` trait for a given struct.
pub fn impl_get_fields(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        syn::Data::Struct(data_struct) => &data_struct.fields,
        _ => panic!("GetFields can only be derived for structs"),
    };

    let field_names: Vec<_> = fields
        .iter()
        .map(|field| match &field.ident {
            Some(ident) => quote::quote! { stringify!(#ident) },
            None => panic!("GetFields can only be derived for named fields"),
        })
        .collect();

    let gen = quote::quote! {
        impl GetFields for #name {
            fn get_fields() -> Vec<&'static str> {
                vec![#(#field_names),*]
            }
        }
    };

    gen.into()
}
