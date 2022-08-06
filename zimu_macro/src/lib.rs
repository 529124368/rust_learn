extern crate proc_macro;
use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{self, ext::IdentExt, parse_macro_input, DeriveInput};

fn impl_zimu_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! (
        impl  #name {
            fn hello_world() {
                println!("我的第一个派生宏");
            }
            fn test(&self) {
                println!("我的第一个派生宏");
            }
        }
    );
    gen.into()
}

#[proc_macro_derive(zimuMacro)]
pub fn zimu_macro_derive(input: TokenStream) -> TokenStream {
    // let ast = syn::parse(input).unwrap();
    let ast = parse_macro_input!(input as DeriveInput);

    let d = match &ast.data {
        syn::Data::Struct(s) => &s.fields,
        _ => todo!(),
    };
    let mut boxs = Vec::new();
    for i in d.iter() {
        let name = i.ident.as_ref().unwrap();
        let ty = &i.ty;
        let method_name: proc_macro2::TokenStream =
            format!("get_{}", name.unraw().to_string()).parse().unwrap();
        boxs.push(quote! (
            fn #method_name(&self) -> #ty {
                self.#name.clone()
            }
        ));
    }

    let name = &ast.ident;

    let gen = quote! (
        impl  #name {
            fn new() ->Self {
                #name::default()
            }
            fn hello_world() {
                println!("我的第一个派生宏");
            }
            #(#boxs)*
        }
    );
    gen.into()
    //impl_zimu_macro(&ast)
}
