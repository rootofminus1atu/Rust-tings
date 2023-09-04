use std::fmt::Debug;

pub fn display() {
    let option1 = Some(5);
    println!("option1: {:?}", option1.unwrap_or(0));
    println!("option1: {:?}", option1.unwrap_or_else(|| 0));

    let option2 = None;
    println!("option2: {:?}", option2.unwrap_or(0));
    println!("option2: {:?}", option2.unwrap_or_else(|| 0));




    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        sort_operations.push(&value);
        r.width * r.height
    });
    println!("{:#?}", list);
    println!("{:#?}", sort_operations);


}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}





#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // which should I use? what's the difference? is there any difference?
        //user_preference.unwrap_or(self.most_stocked())
        user_preference.unwrap_or_else(|| self.most_stocked())
        // this one is lazy which is better than unnecessarily computing it
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

