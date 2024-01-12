use async_trait::async_trait;

macro_rules! generate_dollar_str {
    () => { "" };
    ($($name:ident),*) => {
        [$(stringify!($name),)*]
            .iter()
            .enumerate()
            .map(|(a, _)| format!("${}", a + 1))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

macro_rules! generate_coalesce2 {
    () => { "" };
    ($($name:ident),*) => {
        [$(stringify!($name),)*]
            .iter()
            .enumerate()
            .map(|(i, elem)| format!("{} = COALESCE(${}, {})", elem, i + 1, elem))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn join_with_template<F>(strs: &[&str], template_func: F, separator: &str) -> String
where
    F: Fn((usize, &&str)) -> String,
{
    strs.iter()
        .enumerate()
        .map(template_func)
        .collect::<Vec<_>>()
        .join(separator)
}

macro_rules! count_idents {
    () => { 0 };
    ($single_ident:ident) => { 1 };
    ($_ignored:ident, $($rest:ident),*) => { 1 + count_idents!($($rest),*) };
}


/// ALMOST WORKED
macro_rules! one_two_three {
    () => { ("", 0) } ;
    ($single_ident:ident) => { 
        (
            format!("{} = COALESCE(${}, {})", stringify!($single_ident), 1, stringify!($single_ident)), 
            1
        ) 
    };
    ($first:ident, $($rest:ident),*) => {
        {
            let (rest_str, rest_count) = one_two_three!($($rest),*);

            (
                format!("{}, {} = COALESCE(${}, {})", rest_str, stringify!($first), 1 + rest_count, stringify!($first)),
                1 + rest_count
            )
        }
    };
}


macro_rules! new_try {
    () => { "" };
    ($single_ident:ident) => { format!("{} = COALESCE(${}, {})", stringify!($single_ident), 1, stringify!($single_ident)) };
    ($first:ident, $($rest:ident),*) => {  
        {

            format!("{} = COALESCE(${}, {}), {}", stringify!($first), count_idents!($($rest),*) + 1, stringify!($first), new_try!($($rest),*))
        }
    };
}





macro_rules! __gen_cos_internal {
    ($start:expr, $single_ident:ident) => {
        format!("{} = COALESCE(${}, {})", stringify!($single_ident), $start, stringify!($single_ident))
    };
    ($num:expr, $first:ident, $($rest:ident),*) => {
        format!("{} = COALESCE(${}, {}), {}", stringify!($first), $num, stringify!($first), __gen_cos_internal!($num + 1, $($rest),*))
    };
}

macro_rules! generate_coalesce {
    () => { "" };
    ($($idents:ident),*) => {
        __gen_cos_internal!(1, $($idents),*) 
    };
}





#[async_trait]
trait CrudOperations {
    async fn insert_one(pool: &sqlx::PgPool);
}

/// NOTE:
/// 
/// This assumes that the PK is the 1st field in the struct
macro_rules! generate_table {
    ($struct_name:ident { $pk_name:ident: $pk_type:ty, $($field_name:ident: $field_type:ty),* }, table_name: $table_name:ident) => {

        #[derive(Debug, sqlx::FromRow)]
        struct $struct_name {
            $pk_name: $pk_type,
            $($field_name: $field_type),*
        }

        impl $struct_name {
            const TABLE_NAME: &str = stringify!($table_name);
            // const COA: &str = generate_coalesce!($pk_name, $($field_name),*);

            fn new($pk_name: $pk_type, $($field_name: $field_type),*) -> Self {
                $struct_name {
                    $pk_name,
                    $($field_name),*
                }
            }

            pub fn insert_one(pool: &str, $($field_name: $field_type),*) /* -> Result<Self, sqlx::Error> */ {
                let sql = format!(
                    "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
                    Self::TABLE_NAME,
                    stringify!($($field_name),*),
                    join_with_template(
                        &[$(stringify!($field_name),)*],
                        |(i, _)| format!("${}", i + 1),
                        ", "
                    )
                );

                println!("generated sql: {}", sql);
                
                // let result = sqlx::query_as::<_, Self>(&sql)
                //     $(.bind($field_name))*
                //     .fetch_one(pool)
                //     .await?;

                // Ok(result)
            }

            pub fn get_all(pool: &str) {
                let sql = format!(
                    "SELECT * FROM {}",
                    Self::TABLE_NAME
                );

                println!("generated sql: {}", sql);
            }

            pub fn delete_by_pk(pool: &str, $pk_name: $pk_type) {
                let sql = format!(
                    "DELETE FROM {} WHERE {} = $1 RETURNING *",
                    Self::TABLE_NAME,
                    stringify!($pk_name)
                );

                println!("generated sql: {}", sql);
            }

            pub fn edit(pool: &str, $pk_name: $pk_type, $($field_name: Option<$field_type>),*) {
                let normal_fields = [$(stringify!($field_name),)*];
                let normal_fields_count = normal_fields.len();

                let coalesce_stuff = join_with_template(
                    &normal_fields, 
                    |(i, elem)| format!("{} = COALESCE(${}, {})", elem, i + 1, elem),
                    ", "
                );

                let sql = format!(
                    "UPDATE {} SET {} WHERE {} = ${} RETURNING *",
                    Self::TABLE_NAME,
                    coalesce_stuff,
                    stringify!($pk_name),
                    normal_fields_count + 1
                );

                println!("generated sql: {}", sql);
            }
        }
    };
}

generate_table!(
    Person {
        id: i32,
        name: String,
        age: i32 
    }, 
    table_name: person
);



pub trait Model {
    fn hi();
}

impl Model for Person {
    fn hi() {
        
    }
}





macro_rules! count_idents2 {
    () => { 0 };
    ($($name:ident),*) => {
        {
            let counter = [$(stringify!($name),)*];
            counter.len()
        }
    }
}




macro_rules! generate_placeholders {
    () => { "" };
    ($($field_name:ident),*) => {
        {
            let amount = count_idents!($($field_name),*);

            (1..=amount).map(|i| format!("${}", i)).collect::<Vec<_>>().join(", ")
        }
    };
}




fn main() {
    // Create an instance of the generated struct
    

    let person = Person::new(2, "John".to_string(), 30);

    // Print the struct for debugging
    println!("{:?}", person);

    let idk = generate_placeholders!(hi, lol);
    println!("generated {}", idk);


    let res = generate_dollar_str!(hi, hi, thhhwh);
    println!("got {}", res);


    let pool = "pool";

    // let sql = Person::insert_one("hi", "nammeee".to_string(), 4);
    let _ = Person::edit(pool, 22, Some("hi".into()), Some(99));
    let _ = Person::delete_by_pk(pool, 23);
    let _ = Person::get_all(pool);
    let _ = Person::insert_one(pool, "namee".into(), 80);

    let hh = generate_coalesce!(hello, there, human);
    println!("{:?}", hh);



    let n = count_idents!(hi, lol, idk);
}
