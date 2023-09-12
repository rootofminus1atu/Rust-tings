pub fn display() {
    let mut better_list: LinkedList<i32> = LinkedList::new();
    better_list.push(5);
    better_list.push(6);
    better_list.push(7);
    println!("better_list = {:?}", better_list);

    better_list.push_back(8);
    println!("better_list = {:?}", better_list);



    let mut another_list = LinkedList::new();
    another_list.push_back(1);
    println!("");
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

    fn go_to_back(&mut self) -> &mut Option<Box<Node<T>>> {
        let mut tracker: &mut Option<Box<Node<T>>> = &mut self.head;

        while tracker.is_some() {
            tracker = &mut tracker.as_mut().unwrap().next;
        }
        
        tracker
    }

    fn push_back(&mut self, data: T) {
        let tracker = self.go_to_back();
        *tracker = Some(Node::new_boxed(data));
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












use List::{Cons, Nil};
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

fn archive_lol() {

    struct MyStruct {
        field: i32,
    }

    let my_boxed_struct = Box::new(MyStruct { field: 42 });

    // This won't work without explicit dereferencing
    // let value = my_boxed_struct.field;

    // You need to dereference explicitly  // no
    let value = (*my_boxed_struct).field;
    let value2 = my_boxed_struct.field;
    let value3 = &my_boxed_struct.field;
    //let value4 = *my_boxed_struct;
    let value5 = &my_boxed_struct.field;



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
    better_list.push(5);
    better_list.push(6);
    better_list.push(7);
    println!("better_list = {:?}", better_list);

    let mut tracker: &Option<Box<Node<i32>>> = &better_list.head;
    let whatthis = tracker.as_deref();

    if tracker.is_some() {
        println!("Tracker is currently pointing to {:?}", tracker);

        let insides: Option<&Box<Node<i32>>> = match tracker {
            Some(thing) => Some(thing),
            None => None,
        };

        let another = tracker.as_ref();
        // wow this deref coercion is spooky
        let another_one = tracker.as_deref();


        let another_another_one = tracker.as_deref().unwrap().data;
        let another_another_one2: &Box<Node<i32>> = tracker.as_ref().unwrap();

        // = tracker.something... but what? how can I get to the Option only or to something similar
        // how can I move out of a &Some() and grab what's inside of it?
    };

    while let Some(thing) = tracker {
        let thingy = thing.data;
        println!("Tracker is currently pointing to {:?}", tracker);
        println!("Data inside is {:?}", thingy);
        tracker = &thing.next;
    }

    
    println!("better_list = {:?}", better_list);
    



    let what: &Option<String> = &Some(String::from("hello"));

    let why_does_this_work: Option<&String> = match what {
        &Some(ref thing) => Some(thing),
        &None => None,
    };

    let idk = "�";
}

