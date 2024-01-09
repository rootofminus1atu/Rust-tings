use hello_macro::HelloMacro;
use hello_macro_derive::{HelloMacro, TableThing};

#[derive(HelloMacro)]
struct Pancakes;

#[derive(TableThing)]
struct Person {
    id: i32,
    name: String
}


fn main() {
    Person::get_all();
    Person::insert(1, "hi".into());
}
