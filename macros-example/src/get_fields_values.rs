use proc_macro::TokenStream;

/// Implement the `GetFieldValues` trait for a given struct.
pub fn impl_get_fields_values(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        syn::Data::Struct(data_struct) => &data_struct.fields,
        _ => panic!("GetFields can only be derived for structs"),
    };

    let fields_values: Vec<_> = fields
        .iter()
        .map(|field| match &field.ident {
            Some(ident) => quote::quote! { self.#ident },
            None => panic!("GetFieldsValues can only be derived for named fields"),
        })
        .collect();

    let gen = quote::quote! {
        impl GetFieldsValues for #name {
            fn get_fields_values(&self) -> Vec<String> {
                vec![#(#fields_values.to_string()),*]
            }
        }
    };

    gen.into()
}
