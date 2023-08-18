pub fn enums_demo() {
    let a = Shape::Rectangle(Rectangle {
        width: 10.0,
        height: 20.0,
    });

    a.say_hello();
}

#[derive(Debug)]
#[allow(dead_code)]
enum Shape {
    Rectangle(Rectangle),
    Square { side: f32 },
    Circle { radius: f32 },
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: f32,
    height: f32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Square {
    side: f32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Circle {
    radius: f32,
}

impl Shape {
    fn say_hello(&self) {
        println!("Hello, I am a shape");
    }
}




pub fn pennies_demo() {

    let coin = Coin::Penny;
    println!("Value in cents: {}", coin.value_in_cents());

    let coin = Coin::Quarter(State::Alaska);
    println!("Value in cents: {}", coin.value_in_cents());

}

#[derive(Debug)]
#[allow(dead_code)]
enum State {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}


#[allow(dead_code)]
impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
}