use hello_macro::HelloMacro;
use hello_macro_derive::{HelloMacro, TableThing};

use sqlx::{PgPool, Row};

pub struct Data {
    db: PgPool
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(HelloMacro)]
struct Pancakes;




#[derive(TableThing)]
struct Person2 {
    #[pk]
    id: i32
}

#[derive(TableThing)]
#[table_name = "person_lol"]
struct Person {
    #[pk]
    id: i32,
    name: String,
    age: i32
}



fn main() {
    // Person::get_all();
    // Person::insert("hi".into(), 2);

}
