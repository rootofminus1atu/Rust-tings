use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, Fields, DeriveInput, Result, Error, punctuated::Punctuated, Field, token::Comma};

use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row, query_as};


fn get_attr_named(input: &DeriveInput, name: &str) -> Option<String> {
    input.attrs.iter().find_map(|attr| {
        if let Ok(meta) = attr.parse_meta() {
            if let syn::Meta::NameValue(nv) = meta {
                if nv.path.is_ident(name) {
                    if let syn::Lit::Str(name_lit) = nv.lit {
                        return Some(name_lit.value());
                    }
                }
            }
        }
        None
    })
}

/// Returns the fields from a struct with AT LEAST ONE guaranteed field
fn get_struct_fields(input: &DeriveInput) -> Result<&Punctuated<Field, Comma>> {
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                if fields.named.is_empty() {
                    return Err(Error::new_spanned(input, "TableThing requires at least 1 named field"))
                }

                Ok(&fields.named)
            },
            _ => Err(Error::new_spanned(input, "TableThing requires named fields, no tuple or unit structs"))
        },
        _ => Err(Error::new_spanned(input, "TableThing can be derived for structs only"))
    };

    fields
}

fn divide_fields<'a>(fields: &'a Punctuated<Field, Comma>, pk_name: &str) -> (&'a Field, Punctuated<&'a Field, Comma>) {
    let pk_field = fields.iter()
        .find(|field| field.ident.as_ref().unwrap().to_string() == pk_name)
        .expect("How tf is there no pk field");

    let normal_fields =  fields.iter()
        .filter(|field| field.ident.as_ref().unwrap().to_string() != pk_name)
        .collect::<Punctuated<_, Comma>>();

    (pk_field, normal_fields)
}

/// An `idty` token is of the shape `ident: type`, which is often used in method signatures.
fn generate_idty(field: &Field) -> TokenStream {
    let ident = field.ident.as_ref().unwrap();
    let ty = &field.ty;

    quote!(#ident: #ty)
}

pub fn table_impl(input: DeriveInput) -> Result<TokenStream> {
    let all_fields = get_struct_fields(&input)?;

    


    // getting all the important names and other error checks
    let struct_name = &input.ident;

    let table_name = get_attr_named(&input, "table_name")
        .unwrap_or_else(|| struct_name.to_string());

    let pk_name = get_attr_named(&input, "pk")
        .unwrap_or_else(|| all_fields[0].ident.as_ref().unwrap().to_string());




    // dividing fields
    let (pk_field, normal_fields) = divide_fields(all_fields, &pk_name);


    // generating tokens
    let pk_idty_token = generate_idty(pk_field);
    let normal_idty_tokens = normal_fields.iter()
        .map(|field| generate_idty(field))
        .collect::<Vec<_>>();

    let all_idents = all_fields.iter()
        .map(|field| field.ident.as_ref().unwrap())
        .collect::<Vec<_>>();

    let normal_idents = normal_fields.iter()
        .map(|field| field.ident.as_ref().unwrap())
        .collect::<Vec<_>>();

    let pk_ident = pk_field.ident.as_ref().unwrap();

    let normal_types = normal_fields.iter()
        .map(|field| &field.ty)
        .collect::<Vec<_>>();

    


    // generating sql helpers
    let dolar_str = (0..normal_fields.len())
        .map(|num| format!("${}", num + 1))
        .reduce(|acc, cur| acc + ", " + &cur)
        .unwrap_or("".into());
    
    let normal_idents_str = normal_idents
        .iter()
        .map(|ident| ident.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let coalesce_str = normal_fields
        .iter()
        .enumerate()
        .map(|(i, field)| format!("{} = COALESCE(${}, {})", field.ident.as_ref().unwrap(), i + 1, field.ident.as_ref().unwrap()))
        .reduce(|acc, cur| acc + ", " + &cur)
        .unwrap_or("".into());


    // generating sql
    let get_all_sql = format!("SELECT * FROM {}", table_name);

    let insert_sql = format!(
        "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
        table_name,
        normal_idents_str,
        dolar_str
    );

    let delete_sql = format!(
        "DELETE FROM {} WHERE {} = $1 RETURNING *",
        table_name,
        pk_name
    );

    let update_sql = format!(
        "UPDATE {} SET {} WHERE {} = ${} RETURNING *",
        table_name,
        coalesce_str,
        pk_name,
        all_fields.len()
    );

    

    let gen = quote! {
        

        impl<'r> sqlx::FromRow<'r, PgRow> for #struct_name {
            fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
                let result = Self {
                    #(
                        #all_idents: row.try_get(stringify!(#all_idents))?,
                    )*
                };
        
                Ok(result)
            }
        }

        impl #struct_name {
            const TABLE_NAME: &str = #table_name;

            pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
                let results = query_as::<_, Self>(#get_all_sql)
                    .fetch_all(pool)
                    .await?;

                Ok(results)
            }

            pub async fn insert(pool: &PgPool, #(#normal_idents: #normal_types),*) -> Result<Self, sqlx::Error> {
                let result = query_as::<_, Self>(#insert_sql)
                    #(.bind(#normal_idents))*
                    .fetch_one(pool)
                    .await?;

                Ok(result)
            }

            pub async fn update(pool: &PgPool, #pk_idty_token, #(#normal_idty_tokens),*) -> Result<Option<Self>, sqlx::Error> {
                let result = query_as::<_, Self>(#update_sql)
                    #(.bind(#normal_idents))*
                    .bind(#pk_ident)
                    .fetch_optional(pool)
                    .await?;

                Ok(result)
            }

            pub async fn delete(pool: &PgPool, #pk_idty_token) -> Result<Option<Self>, sqlx::Error> {
                let result = query_as::<_, Self>(#delete_sql)   
                    .bind(#pk_ident)
                    .fetch_optional(pool)
                    .await?;
        
                Ok(result)
            }
        }
    };

    Ok(gen.into())
}
