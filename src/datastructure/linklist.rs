
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::fmt::Debug;

/*
 * Purpose implement a doubly link list
 * that will help manage tasks
 * */




/*
 * id to determine a task 
 * rank to determine priority
 * state will need to change to a different struct
*/
#[derive(Debug)]
 struct Task {
    id: u32,
    rank: u32,
    state: i32 // will change to a task struct
}

// struct for the node in linklist
//
#[derive(Debug)]
struct Node {
    node: Rc<RefCell<Task>>,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn new(task: Task) -> Rc<RefCell<Self>> {
        
        Rc::new(RefCell::new(Node {
            node: Rc::new(RefCell::new(task)),
            next: None,
            prev: None,
        }))
    }
}

// the structure of a doubly linked list
struct LinkList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Weak<RefCell<Node>>>,
}

pub fn testing() {
   let task = Task{
       id: 1,
       rank: 1,
       state: 0,
   };

    let node = Node::new(task);
    let b_n = node.borrow();
    println!("{:#?}", b_n);
}


