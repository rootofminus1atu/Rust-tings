pub fn vectors_demo() {
    println!("Vectors demo");

    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    println!("v = {:?}", v);

    for i in &v {
        println!("i = {}", i);
    }

    let mut w = vec![1, 2, 3, 4, 5];
    println!("w = {:?}", w);

    for i in &mut w {
        *i += 50;
    }

    println!("w = {:?}", w);
}