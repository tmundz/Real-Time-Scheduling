use super::Task;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::{Rc, Weak};

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
#[derive(Debug, Clone)]
pub struct Node {
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
#[derive(Debug, Clone)]
pub struct LinkList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Weak<RefCell<Node>>>,
    size: i32,
}
impl LinkList {
    pub fn new() -> Self {
        LinkList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn push_back(&mut self, task: Task) {
        let new_node = Node::new(task);
        match self.tail.take() {
            //Changes value of tail with none while taking ownership
            //matches to a tail if one exists
            Some(prev_tail) => {
                let new_tail = Rc::downgrade(&new_node); // creates a weak reference
                                                         //upgrade the weak reference then borrow_mut so that the next feild can be updated
                                                         //to the new tail.
                prev_tail.upgrade().unwrap().borrow_mut().next = Some(new_node.clone()); //weak
                new_node.borrow_mut().prev = Some(prev_tail);
                self.tail = Some(new_tail);
                self.size += 1;
            }
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
    pub fn len(&self) -> i32 {
        self.size
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    //TODO Complete Traversal over doubly linked list forewards an backwards
    pub fn search_by_node(&self, value: Rc<RefCell<Task>>) -> Option<Rc<RefCell<Node>>> {
        if self.is_empty() {
            println!("empty list");
            return None;
        }
        let mut cur = self.head.clone();
        while let Some(node) = cur {
            let cur_node = node.borrow();
            let task = cur_node.node.borrow();
            if value.borrow().id == task.id {
                return Some(Rc::clone(&node));
            }
            //move to next node
            cur = cur_node.next.clone();
        }
        None
    }

    pub fn pop(&mut self) -> Option<Task> {
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

    pub fn get_head(&self) -> Option<Rc<RefCell<Task>>> {
        Some(self.head.clone().unwrap().borrow().node.clone())
    }

    pub fn display(&self, indent: &str) {
        let mut cur = self.head.clone();
        while let Some(node) = cur {
            let cur_node = node.borrow();
            let task = cur_node.node.borrow();
            println!(
                "{}Task: id={}, rank={}, state={}, size={}",
                indent, task.id, task.rank, task.state, self.size
            );
            cur = cur_node.next.clone();
        }
    }
}
/*
 * may be useful to have a push front push back
*/

#[cfg(test)]
mod tests {
    use super::{LinkList, Task};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn basic_functions() {
        // Create and add a bunch of tasks
        let tasks = (1..=5).map(|i| Task::new(i, i, 0)).collect::<Vec<_>>();

        let mut ll = LinkList::new();
        assert_eq!(ll.size, 0);

        // Push tasks into the linked list
        ll.push_back(tasks[0].clone());
        ll.push_back(tasks[1].clone());
        ll.push_back(tasks[2].clone());
        ll.push_back(tasks[3].clone());
        ll.push_back(tasks[4].clone());

        // Check the size of the linked list
        assert_eq!(ll.size, 5);
    }

    #[test]
    fn empty_functions() {
        // Create and add a bunch of tasks
        let tasks = (1..=5).map(|i| Task::new(i, i, 0)).collect::<Vec<_>>();

        let mut ll = LinkList::new();
        assert!(ll.is_empty());

        // Push tasks into the linked list
        ll.push_back(tasks[0].clone());

        // Check if the linked list is empty
        assert!(!ll.is_empty());

        // Push more tasks
        ll.push_back(tasks[1].clone());
        ll.push_back(tasks[2].clone());
        ll.push_back(tasks[3].clone());
        ll.push_back(tasks[4].clone());

        // Check if the linked list is still not empty
        assert!(!ll.is_empty());

        // Check the size of the linked list
        assert_eq!(ll.size, 5);
    }

    #[test]
    fn find_functions() {
        // Create and add a bunch of tasks
        let tasks = (1..=5).map(|i| Task::new(i, i, 0)).collect::<Vec<_>>();

        let mut ll = LinkList::new();

        // Search for nodes in an empty linked list
        assert!(ll
            .search_by_node(Rc::new(RefCell::new(tasks[0].clone())))
            .is_none());

        // Push tasks into the linked list
        ll.push_back(tasks[0].clone());
        ll.push_back(tasks[1].clone());
        ll.push_back(tasks[2].clone());

        // Search for nodes that exist in the linked list
        let found_node = ll.search_by_node(Rc::new(RefCell::new(tasks[0].clone())));
        assert!(found_node.is_some());
        assert_eq!(found_node.unwrap().borrow().node.borrow().id, tasks[0].id);

        // Push more tasks
        ll.push_back(tasks[3].clone());
        ll.push_back(tasks[4].clone());

        // Search for nodes that don't exist in the linked list
        assert!(ll
            .search_by_node(Rc::new(RefCell::new(Task::new(100, 100, 0))))
            .is_none());
    }

    #[test]
    fn pop_functions() {
        // Create and add a bunch of tasks
        let tasks = (1..=5).map(|i| Task::new(i, i, 0)).collect::<Vec<_>>();

        let mut ll = LinkList::new();
        assert_eq!(ll.size, 0);

        // Push tasks into the linked list
        ll.push_back(tasks[0].clone());
        ll.push_back(tasks[1].clone());
        ll.push_back(tasks[2].clone());
        ll.push_back(tasks[3].clone());
        ll.push_back(tasks[4].clone());
        assert_eq!(ll.size, 5);

        // Pop tasks and check the size
        assert_eq!(ll.pop().unwrap().id, 1);
        assert_eq!(ll.pop().unwrap().id, 2);
        assert_eq!(ll.size, 3);
        assert_eq!(ll.pop().unwrap().id, 3);
        assert_eq!(ll.pop().unwrap().id, 4);
        assert_eq!(ll.size, 1);
        assert_eq!(ll.pop().unwrap().id, 5);
        assert_eq!(ll.size, 0);
    }
}
