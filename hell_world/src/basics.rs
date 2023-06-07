pub fn basics_demo() {
    let x = 1;
    println!("Hello!");
    println!("{x}");

    let mut y = 1;
    println!("{y}");
    y = 2;
    println!("{y}");

    let a = 2.87;
    let a2: i32 = a as i32;
    println!("{a} => {a2}");

    let num = 6.0;
    let den = 5.0;
    println!("{num} / {den} = {}", num / den);

    // let truth = 5 < 6;
}

pub fn loops_demo() {
    for i in 0..10 {
        println!("{i} l");
    }

    let mut number = 0;
    
    loop {
        number += 1;
        println!("{}", number);
        
        if number >= 10 {
            break;
        }
    }
}