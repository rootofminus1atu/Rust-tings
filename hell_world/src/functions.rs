
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    return a * b
}

fn length(s: &&str) -> usize {
    s.len()
}

fn other_length(s: &String) -> usize {
    s.len()
}

fn swap(a: &mut i32, b: &mut i32) {
    println!("swapping {} and {}", a, b);
    let temp = *a;
    *a = *b;
    *b = temp;
}

pub fn functions_demo() {
    let a = 5;
    let b = 6;

    let sum = add(a, b);
    println!("{} + {} = {}", a, b, sum);

    let product = multiply(a, b);
    println!("{} * {} = {}", a, b, product);


    let s = "Hello!";
    println!("length of {} = {}", s, length(&s));

    let streng = String::from(s);
    println!("length of {} = {}", streng, other_length(&streng));


    let mut n = 5;
    let mut m = 6;

    println!("n = {}, m = {}", n, m);
    swap(&mut n, &mut m);
    println!("n = {}, m = {}", n, m);
}

