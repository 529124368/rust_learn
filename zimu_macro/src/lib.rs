extern crate proc_macro;
use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{self, ext::IdentExt, parse_macro_input, DeriveInput, TypeParen};

#[proc_macro_derive(zimuMacro)]
pub fn zimu_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let d = match &ast.data {
        syn::Data::Struct(s) => &s.fields,
        _ => todo!(),
    };
    let mut boxs = Vec::new();
    for i in d.iter() {
        let name = i.ident.as_ref().unwrap();
        let ty = &i.ty;
        //println!("@@{:#?}", ty);
        let ss = match ty {
            syn::Type::Path(a) => a.path.leading_colon,
            _ => todo!(),
        };
        println!("@@{:#?}", ss);

        //
        let method_name: proc_macro2::TokenStream =
            format!("get_{}", name.unraw().to_string()).parse().unwrap();
        let method_name2: proc_macro2::TokenStream =
            format!("set_{}", name.unraw().to_string()).parse().unwrap();
        boxs.push(quote! (
            //get
            fn #method_name(&self) -> #ty {
                self.#name.clone()
            }
            //set
            fn #method_name2(&mut self,v:#ty){
                self.#name =v
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
}
