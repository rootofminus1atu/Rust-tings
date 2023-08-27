pub fn display() {
    println!("more_ownership.rs");

    let result: &i32;
    result = &6;

    {
        let number_list = vec![34, 50, 25, 100, 65];

        // result = largest_i32(&number_list);  // use clone() to outlize the list
        println!("The largest number is {}", result);
    }
    
    println!("The largest number is {}", result);
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}