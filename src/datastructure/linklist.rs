
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::fmt::Debug;

/*
 *
 * [None] <=> [Node1:Task1] <=> [Node2:Task2] <=> [...] [NodeN:taskN]<=> [None]
 *         ^                                                          ^
 *         |                                                          |
 *       Head                                                        Tail
 * Tail is a reference to NodeN and the next aka None
 * Head is a reference to Node1 and the previous aka None
 * Purpose implement a doubly link list
 * that will help manage tasks
 * */




/*
 * id to determine a task 
 * rank to determine priority
    ll.push_back(task);
 * state will need to change to a different struct
*/
#[derive(Debug, Clone)]
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
#[derive(Debug)]
struct LinkList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Weak<RefCell<Node>>>,
}
impl LinkList {
    pub fn new() -> Self{
        LinkList {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, task: Task) { 
        let new_node = Node::new(task);
        match self.tail.take() {
            //Changes value of tail with none while taking ownership
            //matches to a tail if one exists
            Some(prev_tail) => {
                let new_tail = Rc::downgrade(&new_node);// creates a weak reference
                //upgrade the weak reference then borrow_mut so that the next feild can be updated
                //to the new tail.
                prev_tail.upgrade().unwrap().borrow_mut().next = Some(new_node.clone());                                                                                         //weak 
                new_node.borrow_mut().prev = Some(prev_tail);
                self.tail = Some(new_tail);
            },
            //else there is no tail doubly link list is empty [Head:None] <=> [Tail:None]
            None => {
                /* [None] <=> [Node1:task1] <=> [None]
                           ^                 ^
                          Head              Tail
                */
                self.head = Some(new_node.clone()); //make the head be the new_node
                // make the tail be a weak reference to the same node
                self.tail = Some(Rc::downgrade(&new_node)); 
               
            }
        }
    }


    //TODO Complete Traversal over doubly linked list forewards an backwards
    //functions will be traversal and reverse travseral 
    
}
/*
 * impl functionalities needed
 * insert
 * pop
 * find/traverse 
*/

pub fn testing() {
    let task = Task{
        id: 1,
        rank: 1,
        state: 0,
    };
    
    let mut ll = LinkList::new();
    ll.push_back(task.clone());
    println!("push one\n{:#?}", ll);
    ll.push_back(task);
    println!("\n\npush two\n{:#?}", ll);

}


