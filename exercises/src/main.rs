mod fibonacci;
mod mult_table;


fn main() {
    // fibonacci::display();
    mult_table::display();



    let s = String::from("hello");
    let sr = &s;  // "a reference is a variable that contains the address of another variable" ~ my mind

    take_str_ref(sr);

    // shouldn't sr be invalid here? wasn't it dropped in get_len?
    println!("{}", sr);

    let a = 10;
    let b = a;

    println!("a: {}, b: {}", a, b);




    let mut s = String::from("hello");
    let sr = &mut s;

    take_str_mut_ref(sr);

    println!("{}", sr);
}

#[allow(dead_code)]
fn take_str_ref(sr: &String) {
    println!("{} should now be mine", sr);
}

#[allow(dead_code)]
fn take_str_mut_ref(sr: &mut String) {
    println!("{} should now be mine", sr);
}

#[allow(dead_code)]
fn take_str(s: String) {
    println!("{} should now be mine", s);
}


