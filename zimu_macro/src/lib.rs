extern crate proc_macro;
use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

fn impl_zimu_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! (
        impl  #name {
            fn hello_world() {
                println!("我的第一个派生宏")
            }
        }
    );
    gen.into()
}

#[proc_macro_derive(zimuMacro)]
pub fn zimu_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_zimu_macro(&ast)
}
