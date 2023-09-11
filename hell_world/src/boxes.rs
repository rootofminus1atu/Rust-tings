use std::ops::Deref;

pub fn display() {
    println!("Hello, world!");

    let n = 5;
    let point = Point { x: 1, y: 2 };

    let ref_n = &n;
    let ref_point = &point;

    borrow_point(&point);

    let boxed_n = Box::new(n);

    let point = Point { x: 1, y: 2 };
    let boxed_point = Box::new(point);

    borrow_point(&*boxed_point);
    borrow_point(boxed_point.deref());
    
    let a = boxed_point.deref();
    let a = *boxed_point;

    // println!("boxed_n = {:?}", boxed_point);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn borrow_point(point: &Point) {
    println!("borrow_point: {:?}", point);
}

fn take_point(point: Point) {
    println!("take_point: {:?}", point);
}