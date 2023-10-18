

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
    size: i32,
}
impl LinkList {
    pub fn new() -> Self{
        LinkList {
            head: None,
            tail: None,
            size: 0
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
                self.size += 1;
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
                self.size += 1;
               
            }
        }
    }

    //TODO Complete Traversal over doubly linked list forewards an backwards
    fn head_traversal(&self) {
        if self.size == 0 {
            println!("empty list");
            return;
        }
        let mut cur = self.head.clone();

        while let Some(node) = cur {
            let cur_node = node.borrow();
            let task = cur_node.node.borrow();
            println!("{}", task.id);

            //move to next node
            cur = cur_node.next.clone();
        }

    }

    
    fn pop(&mut self) -> Option<Task> {
        self.head.take().map(|prev_head| {
            self.head = prev_head.borrow().next.clone();
            if let Some(ref new_head) = self.head {
                new_head.borrow_mut().prev = None;
            }
            let task = prev_head.borrow().node.borrow().clone();

            self.size -= 1;
            task

        })
    }
}
/*
 * impl functionalities needed
 * insert
 * pop
 * find/traverse 
*/

pub fn testing() {

     // Create and add a bunch of tasks
    let tasks = vec![
        Task { id: 1, rank: 1, state: 0 },
        Task { id: 2, rank: 2, state: 0 },
        Task { id: 3, rank: 3, state: 0 },
        // Add more tasks as needed
    ];
    
    let mut ll = LinkList::new();
    //ll.push_back(task.clone());
    ll.push_back(tasks[0].clone());
    ll.push_back(tasks[1].clone());
    ll.push_back(tasks[2].clone());
    ll.head_traversal();
    
    println!("pop");
    ll.pop();
    ll.head_traversal();

    println!("pop");
    ll.pop();
    ll.head_traversal();
    println!("pop");
    ll.pop();
    ll.head_traversal();
    //ll.push_back(task);
    println!("pop");
    ll.pop();
    ll.head_traversal();
}


