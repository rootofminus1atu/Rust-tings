use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct MaxQueue<T> {
    max: usize,
    pub list: VecDeque<T>,
}

impl<T> MaxQueue<T> {
    pub fn new(max: usize) -> MaxQueue<T> {
        MaxQueue {
            max,
            list: VecDeque::with_capacity(max),
        }
    }

    pub fn push_front(&mut self, item: T) {
        if self.list.len() == self.max {
            self.list.pop_back();
        }
        self.list.push_front(item);
    }

    pub fn push_back(&mut self, item: T) {
        if self.list.len() == self.max {
            self.list.pop_front();
        }
        self.list.push_back(item);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.list.pop_back()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }

    pub fn get_all(&self) -> &VecDeque<T> {
        &self.list
    }
}