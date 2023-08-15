use std::iter::Iterator;

pub fn array_demo() {
    let arr = [1, 3, 66];

    for (i, &n) in arr.iter().enumerate() {
        println!("arr[{}] = {}", i, n);
    }

    println!("The usual array = {:?}", arr);



    let numbers = [1; 5];

    println!("Array with default args = {:?}", numbers);



    let fruits: [&str; 3] = ["appl", "orang", "bana"];

    for i in 0..arr.len() {
        println!("fruit {} = {}", i, fruits[i]);
    }
    println!("all fruits = {:?}", fruits);

}

pub fn slices_demo() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    let id1 = 2;
    let id2 = 5;

    let slice = &arr[id1..id2];

    println!("slice from {} to {} = {:?}", id1, id2, slice);



    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {:?}", a);

    let s: &[i32] = &a[2..4];

    println!("s: {:?}", s);
}



pub fn structs_demo() {
    
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
        height: u8
    }

    let p1 = Person {
        name: String::from("John"),
        age: 32,
        height: 180
    };

    println!("Person = {}", p1.name);
    println!("Person = {:?}", p1);
    println!("Person = {:#?}", p1);
}