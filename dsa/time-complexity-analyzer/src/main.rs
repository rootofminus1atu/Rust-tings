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



// fn compare(f1, f2)  // this would be an iterator (close to a generator), so that we could iterate and generate a sequence of f1/f2 to see if it converges to 1

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

fn to_be_compared_with(n: usize, counter: &mut usize) {
    for i in 0..n {
        for j in i..n {
            *counter += 1;
        }
    }
}

fn main() {
    for ratio in log_n.compare(to_be_compared_with, 0) {
        println!("ratio: {}", ratio);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    println!("Hello, world!");
}
