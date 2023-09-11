pub fn display() {
    let b = Box::new(5);
    //multiply_by_two(&mut b);
    println!("b = {}", b);
}

fn multiply_by_two(b: &mut Box<i32>) {
    **b *= 2;
}