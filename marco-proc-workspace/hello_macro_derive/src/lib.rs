extern  crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Fields, DeriveInput};


#[proc_macro_derive(TableThing, attributes(table_name, pk))]
pub fn generate_table(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();


    generate_table_impl(&ast)
}

fn get_attr_named(input: &DeriveInput, name: &str) -> Option<String> {
    input.attrs.iter()
    .find_map(|attr| {
        if let Ok(meta) = attr.parse_meta() {
            if let syn::Meta::NameValue(nv) = meta {
                if nv.path.is_ident(name) {
                    if let syn::Lit::Str(table_lit) = nv.lit {
                        return Some(table_lit.value());
                    }
                }
            }
        }
        None
    })
}

fn generate_table_impl(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;

    let table_name = get_attr_named(&input, "table_name")
        .unwrap_or_else(|| name.to_string());

    let pk_name: String = get_attr_named(&input, "pk").unwrap();


    let Data::Struct(data_struct) = &input.data else { unimplemented!("only structs for now") }; 
    let Fields::Named(fields)  = &data_struct.fields else { unimplemented!("only named fields") };
    let fields = &fields.named;


    let idents_and_types = fields
        .iter()
        .map(|f| {
            let ident = f.ident.as_ref().expect("Named fields are expected");
            let ty = &f.ty;
            (ident, ty)
        })
        .collect::<Vec<_>>();

    let indent_and_types_tokens = idents_and_types
        .iter()
        .map(|(ident, ty)| quote!(#ident: #ty))
        .collect::<Vec<_>>();

    let non_pk_idents_and_types = idents_and_types.iter()
        .filter(|(ident, _)| ident.to_string() != pk_name)
        .collect::<Vec<_>>();

    let non_pk_idents_and_types_tokens = non_pk_idents_and_types
        .iter()
        .map(|(ident, ty)| quote!(#ident: #ty))
        .collect::<Vec<_>>();

    let (pk_ident, pk_type) = idents_and_types.iter()
        .find(|(ident, _)| ident.to_string() == pk_name)
        .unwrap_or(&idents_and_types.get(0).unwrap());

    let pk_ident_and_type_token = quote!(#pk_ident: #pk_type);

    


    let dolar_str = (0..idents_and_types.len())
        .map(|num| format!("${}", num + 1))
        .reduce(|acc, cur| acc + ", " + &cur).unwrap_or("".into());

    let coalesce_str = idents_and_types.iter()
        .enumerate()
        .map(|(i, (ident, _))| format!("{} = COALESCE(${}, {})", ident, i + 1, ident))
        .reduce(|acc, cur| acc + ", " + &cur).unwrap_or("".into());

    let get_all_sql = format!("SELEC * FROM {}", table_name);

    let gen = quote! {
        impl #name {
            const TABLE_NAME: &str = #table_name;

            pub fn get_all() {
                println!("hello my name is {} and my fields are", stringify!(#name));
                #(println!("- {}", stringify!(#indent_and_types_tokens));)*
            
                let sql = #get_all_sql;
            }

            pub fn insert(#(#non_pk_idents_and_types_tokens),*) {
                println!("hi {}", #dolar_str);
                println!("hi {}", #coalesce_str);
            }

            pub fn update(#pk_ident_and_type_token, #(#non_pk_idents_and_types_tokens),*) {

            }

            pub fn delete(#pk_ident_and_type_token) {
                
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
