#[derive(Debug)]
#[allow(dead_code)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T, next: Option<Box<Node<T>>>) -> Self {
        Self { value, next, }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct List<T> {
    head: Option<Box<Node<T>>>,
}
 
impl<T> List<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take()
        });
        self.head = Some(new_node);
    }
}



pub fn display() {
    println!("lists_questionmark.rs");
    let mut list = List::new();
    list.push(1);
    println!("{:?}", list);
    list.push(1);
    println!("{:?}", list);
}