use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};

#[proc_macro_attribute]
pub fn parse_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;
    let fn_name = syn::Ident::new(
        &format!("parse_{}", name.to_string().to_lowercase()),
        name.span(),
    );
    let impl_parseable = if attr.to_string().contains("make_parseable") {
        quote! {
            #[cfg(test)]
            impl crate::prelude::Parseable for #name {}
        }
    } else {
        quote! {}
    };
    let generated = quote! {
        #[cfg_attr(test, derive(PartialEq))]
        #input // keep struct

        #impl_parseable

        #[test]
        fn #fn_name() {
            crate::prelude::test_parsing::<#name>();
        }
    };
    generated.into()
}

#[proc_macro_derive(Parseable)]
pub fn parseable_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_parseable(&ast)
}

fn impl_parseable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    quote! {
        impl ::helldivers2_rs::prelude::Parseable for #name {}
    }
    .into()
}
