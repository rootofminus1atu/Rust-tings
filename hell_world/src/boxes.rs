pub fn display() {
    
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let linkedlist = LinkedList {
        head: Some(Box::new(Node {
            data: 1,
            next: Some(Box::new(Node {
                data: 2,
                next: None,
            }))
        }))
    };

    let mut better_list: LinkedList<i32> = LinkedList::new();
    println!("better_list = {:?}", better_list);
    better_list.push(5);
    better_list.pull();
    better_list.pull();
    


}
use List::{Cons, Nil};
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node { data, next: None }
    }

    fn new_boxed(data: T) -> Box<Self> {
        Box::new(Node { data, next: None })
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> where T: std::fmt::Debug {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, data: T) {
        let mut new_node = Node::new_boxed(data);

        match self.head.take() {
            None => self.head = Some(new_node),
            Some(old_head) => {
                new_node.next = Some(old_head);
                self.head = Some(new_node);
            }
        }
    }

    fn pull(&mut self) {
        /*
        if self.head.is_none() {
            return;
        }

        let first_node = self.head.take().unwrap();
        self.head = first_node.next;
        */
        match self.head.take() {
            None => return,
            Some(old_head) => {
                self.head = old_head.next;
            }
        }
    }

    fn append(&mut self, data: T) {
        if self.head.is_none() {
            self.head = Some(Node::new_boxed(data));
            return;
        }

        let mut curr = &mut self.head;
        while curr.is_some() {
            curr = &mut curr.as_mut().unwrap().next;
        }
        
        // curr.unwrap().next = Some(Node::new_boxed(data));


        let mut curr: &mut Option<Box<Node<T>>> = &mut self.head;
        while curr.is_some() && curr.as_ref().unwrap().next.is_some() {
            curr = &mut curr.as_mut().unwrap().next;
        }

        if curr.is_none() {
            self.head = Some(Node::new_boxed(data));
        } else {
            curr.as_mut().unwrap().next = Some(Node::new_boxed(data));
        }


    }
}
