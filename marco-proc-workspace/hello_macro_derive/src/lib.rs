extern  crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Fields, Type};


#[proc_macro_derive(TableThing)]
pub fn generate_table(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    generate_table_impl(&ast)
}

fn generate_table_impl(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;
    
    let Data::Struct(data_struct) = &input.data else { unimplemented!("only structs for now") }; 
    let Fields::Named(named_fields)  = &data_struct.fields else { unimplemented!("only named fields") };

    let ident_names: Vec<&syn::Ident> = named_fields.named.iter().map(|field| {
        let field_name = field.ident.as_ref().expect("Named fields are expected");
        field_name
    }).collect::<Vec<_>>();
    

    let gen = quote! {
        impl #name {
            pub fn get_all() {
                println!("hello my name is {} and my fields are", stringify!(#name));
                #(println!("- {}", stringify!(#ident_names));)*
            }
        }
    };

    gen.into()
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
