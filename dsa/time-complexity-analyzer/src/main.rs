#![allow(unused_variables)]


trait Comparable {
    fn compare<F2>(self, f2: F2, n_start: usize) -> CompareIterator<Self, F2>
    where
        Self: Sized + Fn(usize, &mut usize),
        F2: Fn(usize, &mut usize);
}

impl<F1> Comparable for F1
where
    F1: Fn(usize, &mut usize),
{
    fn compare<F2>(self, f2: F2, n_start: usize) -> CompareIterator<Self, F2>
    where
        F2: Fn(usize, &mut usize) 
    {
        CompareIterator::new(self, f2, n_start)
    }
}



struct CompareIterator<F1, F2>
where 
    F1: Fn(usize, &mut usize),
    F2: Fn(usize, &mut usize),
{
    f1: F1, 
    f2: F2,
    n: usize,
    // max: usize
}

impl<F1, F2> CompareIterator<F1, F2>
where 
    F1: Fn(usize, &mut usize),
    F2: Fn(usize, &mut usize),
{
    fn new(f1: F1, f2: F2, start_n: usize) -> Self {
        CompareIterator {
            f1, f2, n: start_n
        }
    }
}

impl<F1, F2> Iterator for CompareIterator<F1, F2>
where 
    F1: Fn(usize, &mut usize),
    F2: Fn(usize, &mut usize),
{
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut f1_count = 0;
        let mut f2_count = 0;

        (self.f1)(self.n, &mut f1_count);
        (self.f2)(self.n, &mut f2_count);
        
        self.n += 1;

        Some(f1_count as f64 / f2_count as f64)
    }
}

fn compare<F1, F2>(f1: F1, f2: F2, start_n: usize) -> CompareIterator<F1, F2> 
where 
    F1: Fn(usize, &mut usize),
    F2: Fn(usize, &mut usize),
{
    CompareIterator::new(f1, f2, start_n)
}

fn n_squared(n: usize, counter: &mut usize) {
    for i in 0..n {
        for j in 0..n {
            *counter += 1;
        }
    }
}

fn log_n(n: usize, counter: &mut usize) {
    let mut i = n;
    while i > 1 {
        i /= 2;
        *counter += 1;
    }
}


macro_rules! n_k_log_n {
    ($name:ident, $k:expr) => {
        fn $name(n: usize, counter: &mut usize) {
            fn inner(n: usize, counter: &mut usize, depth: usize) {
                if depth == 0 {
                    let mut j = n;
                    while j > 1 {
                        j /= 2;
                        *counter += 1;
                    }
                } else {
                    for _ in 0..n {
                        inner(n, counter, depth - 1);
                    }
                }
            }

            inner(n, counter, $k);
        }
    };
}

n_k_log_n!(n_log_n, 1);

fn to_be_compared_with(n: usize, counter: &mut usize) {
    for i in 0..n {
        for j in i..n {
            *counter += 1;
        }
    }
}

fn n_k_log_n2(k: usize) -> impl Fn(usize, &mut usize) {
    move |n: usize, counter: &mut usize| {
        fn inner(n: usize, counter: &mut usize, depth: usize) {
            if depth == 0 {
                let mut j = n;
                while j > 1 {
                    j /= 2;
                    *counter += 1;
                }
            } else {
                for _ in 0..n {
                    inner(n, counter, depth - 1);
                }
            }
        }

        inner(n, counter, k);
    }
}

/// Note for future self: k from 1 up is valid, 0 shouldn't be used
fn log_n_k(k: usize) -> impl Fn(usize, &mut usize) {
    move |n: usize, counter: &mut usize| {
        fn inner(n: usize, counter: &mut usize, depth: usize) {
            if depth == 0 {
                *counter += 1;
            } else {
                let mut i = n;
                while i > 1 {
                    inner(i, counter, depth - 1);
                    i /= 2;
                }
            }
        }

        inner(n, counter, k);
    }
}


fn main() {
    for ratio in n_squared.compare(n_k_log_n2(1), 3) {
        println!("ratio: {}", ratio);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    println!("Hello, world!");
}
