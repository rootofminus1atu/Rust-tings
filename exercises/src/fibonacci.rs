pub fn display() {
    let n = 30;

    for i in 0..n {
        println!("fibo_faster({}) = {}", i, fibo_faster(i));
    }

    for i in 0..n {
        println!("fibo({}) = {}", i, fibo(i));
    }
}

fn fibo(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    return fibo(n - 1) + fibo(n - 2);
}

fn fibo_faster(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }

    return a;
}