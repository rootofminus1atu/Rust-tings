use std::thread;
use std::time::Duration;

pub fn display() {
/* 
    let list = vec![1, 10, 100];

    for item in ZigzagIter::new(&list).limit_to_steps(5) {
        println!("{}", item);
        thread::sleep(Duration::from_secs(1));
    }*/

    let word = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let rows = 5;

    let list = vec![1, 10, 100, 1000, 10000];

    for item in ZigzagIter::new(&list).limit_to_steps(30) {
        println!("{}", item);
        thread::sleep(Duration::from_millis(100));
    }

}

pub struct ZigzagIter<'a, T> {
    items: &'a [T],
    index: usize,
    direction: bool,

    counter: usize,
    max_steps: usize,
}

impl<'a, T> ZigzagIter<'a, T> {
    pub fn new(items: &'a [T]) -> Self {
        ZigzagIter {
            items,
            index: 0,
            direction: true,
            counter: 0,
            max_steps: usize::MAX
        }
    }

    pub fn limit_to_steps(mut self, steps: usize) -> Self {
        self.max_steps = steps;
        self
    }

    fn tune_index(&mut self) {
        if self.index == self.items.len() - 1 {
            self.direction = false;
        } else if self.index == 0 {
            self.direction = true;
        }

        if self.direction {
            self.index += 1;
        } else {
            self.index -= 1; 
        }
    }

    fn counter_check(&mut self) -> bool {
        self.counter += 1;
        self.counter <= self.max_steps
    }

}

impl<'a, T> Iterator for ZigzagIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.counter_check() {
            return None;
        }

        match self.items.len() {
            0 => None,
            1 => Some(&self.items[0]),
            _ => {
                let item = Some(&self.items[self.index]);
                self.tune_index();
                item
            }
        }
    }
}



