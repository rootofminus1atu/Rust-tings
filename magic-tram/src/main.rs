use std::collections::HashMap;


fn time(n: i32) -> i32 {
    if n == 1 {
        return 0;
    }
    
    if n == 2 {
        return 1;  // represents doing the walk from 1 to 2, the only case where walking is better than taking a tram
    }

    if n % 2 == 0 {
        time(n / 2) + 2
    } else {
        time(n - 1) + 1
    }
}

#[derive(Debug, Clone)]
enum Step {
    Walk,
    Tram
}

impl Step {
    pub fn time(&self) -> i32 {
        match self {
            Step::Walk => 1,
            Step::Tram => 2,
        }
    }
}

fn steps(n: i32) -> Vec<Step> {
    if n == 1 {
        return vec![];
    }

    if n == 2 {
        return vec![Step::Walk]
    }

    if n % 2 == 0 {
        steps(n / 2).into_iter().chain(vec![Step::Tram].into_iter()).collect()
    } else {
        steps(n - 1).into_iter().chain(vec![Step::Walk].into_iter()).collect()
    }
}

fn fib(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    if n == 0 || n == 1 {
        1
    } else {
        if let Some(v) = memo.get(&n) {
            *v
        } else {
            let v = fib(n - 1, memo) + fib(n - 2, memo);
            memo.insert(n, v);
            v
        }
        
    }
}

fn main() {
    let mut memo = HashMap::<i32, i32>::new();
    let n = fib(6, &mut memo);

    println!("Hello, world!");
    for n in 1..=9 {
        println!("{} = {:?} (time = {})", n, steps(n), steps(n).iter().fold(0, |acc, cur| { acc + cur.time() }))
    }
}
