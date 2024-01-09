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
    let Fields::Named(fields)  = &data_struct.fields else { unimplemented!("only named fields") };
    let fields = &fields.named;

    let names_and_types = fields
        .iter()
        .map(|f| {
            let name = f.ident.as_ref().expect("Named fields are expected");
            let ty = &f.ty;
            (name, ty)
        })
        .collect::<Vec<_>>();

    let names_and_types_tokens = names_and_types
        .iter()
        .map(|(name, ty)| quote!(#name: #ty))
        .collect::<Vec<_>>();

    let gen = quote! {
        impl #name {
            pub fn get_all() {
                println!("hello my name is {} and my fields are", stringify!(#name));
                #(println!("- {}", stringify!(#names_and_types_tokens));)*
            }

            pub fn insert(#(#names_and_types_tokens),*) {
                println!("hi")
            }
        }
    };

    
    println!("TOKENS: {}", gen);

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
