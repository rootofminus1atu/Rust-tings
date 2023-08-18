pub fn structs_demo() {
    let person1 = Person::new("John", 32);

    person1.say_hello();

}

#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: &str, age: u8) -> Person {
        Person {
            name: name.to_string(),
            age,
        }
    }

    fn say_hello(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

impl Clone for Person {
    fn clone(&self) -> Self {
        Person {
            name: self.name.clone(),
            age: self.age,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}