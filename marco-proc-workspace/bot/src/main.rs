use hello_macro::HelloMacro;
use hello_macro_derive::{HelloMacro, TableThing};

#[derive(HelloMacro)]
struct Pancakes;

#[derive(TableThing)]
#[table_name = "person_lol"]
#[pk = "id"]
struct Person {
    id: i32,
    name: String,
    age: i32
}


fn main() {
    Person::get_all();
    Person::insert("hi".into(), 2);
}
