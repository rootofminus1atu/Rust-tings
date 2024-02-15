use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::parse::Parse;
use syn::Type;
use syn::{Data, Fields, DeriveInput, Result, Error, punctuated::Punctuated, Field, token::Comma};

use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row, query_as};


/// Returns attrs named `insert your desired name here`
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

/// Returns the fields from a struct
fn get_struct_fields(input: &DeriveInput) -> Result<&Punctuated<Field, Comma>> {
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                if !fields.named.is_empty() {
                    return Ok(&fields.named)
                }

                Err(Error::new_spanned(input, "TableThing requires at least 1 named field"))
            },
            _ => Err(Error::new_spanned(input, "TableThing requires named fields, no tuple or unit structs"))
        },
        _ => Err(Error::new_spanned(input, "TableThing can be derived for structs only"))
    };

    fields
}


struct SqlHorsemen {
    select_sql: String,
    insert_sql: String,
    delete_sql: String,
    update_sql: String
}

struct TableData<'a> {
    struct_ident: &'a Ident,
    table_name: String,
    all_fields: &'a Punctuated<Field, Comma>,
    all_idents: Vec<&'a Ident>,
    pk_field: &'a Field,
    pk_ident: &'a Ident,
    pk_type: &'a Type,
    normal_fields: Punctuated<&'a Field, Comma>,
    normal_idents: Vec<&'a Ident>,
    normal_types: Vec<&'a Type>
}

impl<'a> TableData<'a> {
    fn parse_part2(input: &'a DeriveInput) -> Result<Self> {
        let all_fields = get_struct_fields(&input)?;
        let all_idents = all_fields.iter()
            .map(|field| field.ident.as_ref().unwrap())
            .collect::<Vec<_>>();

        let pk_field = all_fields.iter()
            .find(|f| f.attrs.iter().any(|a| a.path.is_ident("pk")))
            .ok_or(Error::new_spanned(&input, "Mark something with pk pls"))?;
        let pk_ident = pk_field.ident.as_ref().unwrap();
        let pk_type: &syn::Type = &pk_field.ty;
        let pk_name = pk_ident.to_string();

        
        let normal_fields = all_fields.iter()
            .filter(|f| !f.attrs.iter().any(|a| a.path.is_ident("pk")))
            .collect::<Punctuated<_, Comma>>();

        let normal_idents = normal_fields.iter()
            .map(|field| field.ident.as_ref().unwrap())
            .collect::<Vec<_>>();

        let normal_types = normal_fields.iter()
            .map(|field| &field.ty)
            .collect::<Vec<_>>();

        let struct_ident = &input.ident;

        let table_name = get_attr_named(&input, "table_name")
            .unwrap_or_else(|| struct_ident.to_string());

        Ok(TableData {
            struct_ident,
            table_name,
            all_fields,
            all_idents,
            pk_field,
            pk_ident,
            pk_type,
            normal_fields,
            normal_idents,
            normal_types
        }) 
    }

    fn select_sql(&self) -> String {
        "".into()
    }

    pub fn four_sql_horsemen(&self) -> SqlHorsemen {
        let select_sql = format!("SELECT * FROM {}", self.table_name);


        let dolar_str = (0..self.normal_fields.len())
            .map(|num| format!("${}", num + 1))
            .reduce(|acc, cur| acc + ", " + &cur)
            .unwrap_or("".into());
        
        let normal_idents_str = self.normal_idents
            .iter()
            .map(|ident| ident.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let insert_sql = format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
            self.table_name,
            normal_idents_str,
            dolar_str
        );
    

        let delete_sql = format!(
            "DELETE FROM {} WHERE {} = $1 RETURNING *",
            self.table_name,
            self.pk_ident.to_string()
        );
    

        let coalesce_str = self.normal_fields
            .iter()
            .enumerate()
            .map(|(i, field)| format!("{} = COALESCE(${}, {})", field.ident.as_ref().unwrap(), i + 1, field.ident.as_ref().unwrap()))
            .reduce(|acc, cur| acc + ", " + &cur)
            .unwrap_or("".into());

        let update_sql = format!(
            "UPDATE {} SET {} WHERE {} = ${} RETURNING *",
            self.table_name,
            coalesce_str,
            self.pk_ident.to_string(),
            self.all_fields.len()
        );

        SqlHorsemen {
            select_sql,
            insert_sql,
            delete_sql,
            update_sql
        }
    }
}


pub fn table_impl(input: DeriveInput) -> Result<TokenStream> {
    let table_data: TableData<'_> = TableData::parse_part2(&input)?;

    let TableData {
        struct_ident,
        ref table_name,
        all_fields,
        ref all_idents,
        pk_field,
        pk_ident,
        pk_type,
        ref normal_fields,
        ref normal_idents,
        ref normal_types
    } = table_data;

    let SqlHorsemen {
        select_sql,
        insert_sql,
        delete_sql,
        update_sql
    } = &table_data.four_sql_horsemen();


    // str_repr helpers

    let normal_idents_combined = normal_idents.iter()
        .map(|i| format_ident!("{}_{}", table_name, i))
        .collect::<Vec<_>>();

    let pk_ident_combined = format_ident!("{}_{}", table_name, pk_ident);

    let pk_template = format!("**{}: {}**", pk_ident.to_string(), "{}");

    let full_template = normal_idents.iter()
        .fold(pk_template, |acc, cur| format!("{} - `{}: {}`", acc, cur, "{}"));


    // discord specific things
    let table_name_ident = Ident::new(&table_name, Span::call_site());

    let func_all_ident = Ident::new(&format!("{}_{}", table_name, "all"), Span::call_site());
    let func_all_self_str = format!("Self::{}", func_all_ident);

    let func_insert_ident = Ident::new(&format!("{}_{}", table_name, "add"), Span::call_site());
    let func_insert_self_str = format!("Self::{}", func_insert_ident);

    let func_delete_ident = Ident::new(&format!("{}_{}", table_name, "delete"), Span::call_site());
    let func_delete_self_str = format!("Self::{}", func_delete_ident);
    
    let func_update_ident = Ident::new(&format!("{}_{}", table_name, "update"), Span::call_site());
    let func_update_self_str = format!("Self::{}", func_update_ident);


    

    // generating actual code

    let gen = quote! {
        impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for #struct_ident {
            fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
                let result = Self {
                    #(
                        #all_idents: row.try_get(stringify!(#all_idents))?,
                    )*
                };
        
                Ok(result)
            }
        }

        impl #struct_ident {
            const TABLE_NAME: &str = #table_name;

            pub async fn get_all(pool: &sqlx::PgPool) -> Result<Vec<Self>, sqlx::Error> {

                let results = sqlx::query_as::<_, Self>(#select_sql)
                    .fetch_all(pool)
                    .await?;

                Ok(results)
            }

            pub async fn insert(pool: &sqlx::PgPool, #(#normal_idents: #normal_types),*) -> Result<Self, sqlx::Error> {
                let result = sqlx::query_as::<_, Self>(#insert_sql)
                    #(.bind(#normal_idents))*
                    .fetch_one(pool)
                    .await?;

                Ok(result)
            }

            pub async fn update(pool: &sqlx::PgPool, #pk_ident: #pk_type, #(#normal_idents: #normal_types),*) -> Result<Option<Self>, sqlx::Error> {
                let result = sqlx::query_as::<_, Self>(#update_sql)
                    #(.bind(#normal_idents))*
                    .bind(#pk_ident)
                    .fetch_optional(pool)
                    .await?;

                Ok(result)
            }

            pub async fn delete(pool: &sqlx::PgPool, #pk_ident: #pk_type) -> Result<Option<Self>, sqlx::Error> {
                let result = sqlx::query_as::<_, Self>(#delete_sql)   
                    .bind(#pk_ident)
                    .fetch_optional(pool)
                    .await?;
        
                Ok(result)
            }

            pub fn str_repr(&self) -> String {
                format!(#full_template, self.#pk_ident, #(self.#normal_idents),*)
            }

            /// this thing has unused variables lol
            pub async fn testy(
                ctx: Context<'_>,
                #(#normal_idents_combined: #normal_types),*
            ) {

            }

            
            #[poise::command(
                prefix_command,
                slash_command,
                subcommands(/*#func_all_self_str, #func_insert_self_str, #func_delete_self_str, */#func_update_self_str),
                subcommand_required
            )]
            pub async fn #table_name_ident(_: Context<'_>) -> Result<(), Error> {
                Ok(())
            }

            /* 
            /// (OWNER ONLY) see all items
            #[poise::command(prefix_command, slash_command, rename = "all")]
            pub async fn #func_all_ident(ctx: Context<'_>) -> Result<(), Error> {

                let quotes: Vec<#struct_ident> = #struct_ident::get_all(&ctx.data().db).await?;

                ctx.say("db all data embed").await?;

                Ok(())
            }
            
            /// (OWNER ONLY) add an item
            #[poise::command(prefix_command, slash_command, rename = "add")]
            pub async fn #func_insert_ident(
                ctx: Context<'_>,
                #(#normal_idents_combined: #normal_types),*
            ) -> Result<(), Error> {

                let item: #struct_ident = #struct_ident::insert(&ctx.data().db, #(#normal_idents_combined),*).await?;

                ctx.say(format!("Inserted: {}", item.str_repr())).await?;

                Ok(())
            }

            /// (OWNER ONLY) delete an item
            #[poise::command(prefix_command, slash_command, rename = "delete")]
            pub async fn #func_delete_ident(
                ctx: Context<'_>,
                #pk_ident: #pk_type
            ) -> Result<(), Error> {

                let item = #struct_ident::delete(&ctx.data().db, #pk_ident).await?;

                let _ = match item {
                    Some(deleted) => ctx.say(format!("Deleted: {}" , deleted.str_repr())).await?,
                    None => ctx.say(format!("Item with id {} not found", id)).await?
                };

                Ok(())
            }
            */
            
            /// (OWNER ONLY) update an item
            #[poise::command(prefix_command, slash_command, rename = "update")]
            pub async fn #func_update_ident(
                ctx: Context<'_>,
                #pk_ident: #pk_type,
                #(#normal_idents_combined: #normal_types),*
            ) -> Result<(), Error> {

                
                let item = #struct_ident::update(&ctx.data().db, #pk_ident #(,#normal_idents_combined)*).await?;

                let _ = match item {
                    Some(updated) => ctx.say(format!("Updated: {}", updated.str_repr())).await?,
                    None => ctx.say(format!("Quote with id {} not found", id)).await?
                };

                Ok(())
            }
            

        }
    };

    Ok(gen.into())
}
