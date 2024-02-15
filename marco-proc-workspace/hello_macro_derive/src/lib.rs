extern  crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

mod table_impl;
use table_impl::table_impl;


/// Derives the 4 basic CRUD operations for a struct:
/// - `get_all()`
/// - `insert()`
/// - `delete()`
/// - `update()`
/// 
/// All of them have as their 1st argument a `&PgPool` argument.
#[proc_macro_derive(TableThing, attributes(table_name, pk, lol))]
pub fn generate_table(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input);
    
    table_impl(ast)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}







#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name  = &ast.ident;

    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!(
                    "Hello, Macro! My name is {}!",
                    stringify!(#name)
                )
            }
        }
    };

    gen.into()
}
