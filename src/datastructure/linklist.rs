use super::Task;
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::fmt::Debug;

/// [None] <=> [Node1:Task1] <=> [Node2:Task2] <=> [...] [NodeN:taskN]<=> [None]
///         ^                                                          ^
///         |                                                          |
///       Head                                                        Tail
/// Tail is a reference to NodeN and the next aka None
/// Head is a reference to Node1 and the previous aka None
/// Purpose implement a doubly link list
/// that will help manage tasks
///






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

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    //searches task by id
    fn search_by_id(&self, value:u32) -> Option<Rc<RefCell<Node>>> {
        if self.is_empty() {
            println!("empty list");
            return None;
        }
        let mut cur = self.head.clone();
        while let Some(node) = cur {
            let cur_node = node.borrow();
            let task = cur_node.node.borrow();
            if value == task.id {
                return Some(Rc::clone(&node));
            }
            //move to next node
            cur = cur_node.next.clone();
        }
        None

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

#[cfg(test)]
mod test {


    use super::LinkList;
    use super::Task;

    #[test]
    fn basic_functions() {
         // Create and add a bunch of tasks
        let tasks = vec![
            Task { id: 1, rank: 1, state: 0 },
            Task { id: 2, rank: 2, state: 0 },
            Task { id: 3, rank: 3, state: 0 },
            Task { id: 4, rank: 4, state: 0 },
            Task { id: 5, rank: 5, state: 0 },
            // Add more tasks as needed
        ];
        
        let mut ll = LinkList::new();
        assert_eq!(ll.size, 0);
        //ll.push_back(task.clone());
        ll.push_back(tasks[0].clone());
        ll.push_back(tasks[1].clone());
        ll.push_back(tasks[2].clone());
        ll.push_back(tasks[3].clone());
        ll.push_back(tasks[4].clone());
        assert_eq!(ll.size, 5);
    }

    #[test]
    fn empty_functions() {
         // Create and add a bunch of tasks
        let tasks = vec![
            Task { id: 1, rank: 1, state: 0 },
            Task { id: 2, rank: 2, state: 0 },
            Task { id: 3, rank: 3, state: 0 },
            Task { id: 4, rank: 4, state: 0 },
            Task { id: 5, rank: 5, state: 0 },
            // Add more tasks as needed
        ];
        
        let mut ll = LinkList::new();
        assert!(ll.is_empty());
        //ll.push_back(task.clone());
        ll.push_back(tasks[0].clone());
        assert!(!ll.is_empty());
        ll.push_back(tasks[1].clone());
        ll.push_back(tasks[2].clone());
        ll.push_back(tasks[3].clone());
        ll.push_back(tasks[4].clone());
        assert!(!ll.is_empty());
        assert_eq!(ll.size, 5);
    }
    #[test]
    fn find_functions() {
         // Create and add a bunch of tasks
        let tasks = vec![
            Task { id: 1, rank: 1, state: 0 },
            Task { id: 2, rank: 2, state: 0 },
            Task { id: 3, rank: 3, state: 0 },
            Task { id: 4, rank: 4, state: 0 },
            Task { id: 5, rank: 5, state: 0 },
            // Add more tasks as needed
        ];
        
        let mut ll = LinkList::new();
        let id = match ll.search_by_id(0) {
            Some(id) => id.borrow().node.borrow().id,
            None => u32::MAX,
        };
        assert_eq!(id, u32::MAX);
        //ll.push_back(task.clone());
        ll.push_back(tasks[0].clone());
        ll.push_back(tasks[1].clone());
        ll.push_back(tasks[2].clone());
        let midid = match ll.search_by_id(3) {
            Some(midid) => midid.borrow().node.borrow().id,
            None => u32::MAX,
        };
        assert_eq!(midid, 3);
        ll.push_back(tasks[3].clone());
        ll.push_back(tasks[4].clone());
        let id1 = match ll.search_by_id(1) {
            Some(id1) => id1.borrow().node.borrow().id,
            None => u32::MAX,
        };
        assert_eq!(id1, 1);

        let id2 = match ll.search_by_id(100) {
            Some(id2) => id2.borrow().node.borrow().id,
            None => u32::MAX,
        };
        assert_eq!(id2, u32::MAX);
    }

    #[test]
    fn pop_functions() {
         // Create and add a bunch of tasks
        let tasks = vec![
            Task { id: 1, rank: 1, state: 0 },
            Task { id: 2, rank: 2, state: 0 },
            Task { id: 3, rank: 3, state: 0 },
            Task { id: 4, rank: 4, state: 0 },
            Task { id: 5, rank: 5, state: 0 },
            // Add more tasks as needed
        ];
        
        let mut ll = LinkList::new();
        assert_eq!(ll.size, 0);
        ll.push_back(tasks[0].clone());
        ll.push_back(tasks[1].clone());
        ll.push_back(tasks[2].clone());
        ll.push_back(tasks[3].clone());
        ll.push_back(tasks[4].clone());
        assert_eq!(ll.size, 5);
        assert_eq!(ll.pop().unwrap().id, 1);
        assert_eq!(ll.pop().unwrap().id, 2);
        assert_eq!(ll.size, 3);
        assert_eq!(ll.pop().unwrap().id, 3);
        assert_eq!(ll.pop().unwrap().id, 4);
        assert_eq!(ll.size, 1);
        assert_eq!(ll.pop().unwrap().id, 5);
        assert_eq!(ll.size, 0);
    }

    #[test]
    fn outlier_test() {
        let mut ll = LinkList::new();
        match ll.pop() {
            None => (),
            Some(_) => panic!("Popped from empty linked list")
        }
    }
}






