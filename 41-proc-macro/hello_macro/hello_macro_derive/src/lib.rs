// proc macro is crate comes with rust
// import this
extern crate proc_macro;

use proc_macro::TokenStream;
// syntax tree data struct -> string rust code
use quote::quote;
// string rust code -> syntax tree data struct
use syn;

#[(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    return impl_hello_macro(&ast);
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello Macro! My name is {}", stringify!(#name));
            }
        }
    };
    return gen.into();
}