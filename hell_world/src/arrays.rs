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