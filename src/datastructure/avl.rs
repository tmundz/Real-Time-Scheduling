use super::Task;
use std::sync::{Arc, Mutex};
use std::cmp::Ordering;

// An AVL tree is a self-balancing binary search tree. It ensures that the height
// difference between the left and right subtrees of any node (the balance factor)
// does not exceed 1. This balancing property helps maintain the tree's height in
// O(log n), where n is the number of nodes.

#[derive(Debug, Clone)]
pub struct AvlTree {
    val: Task,
    left: Option<Arc<Mutex<AvlTree>>>,
    right: Option<Arc<Mutex<AvlTree>>>,
}

impl AvlTree {
    fn new(task: Task) -> Self {
        AvlTree { 
            val: task, 
            left: None, 
            right: None, 
        }
        
    }
}

//insert
//balance
//Delete
//traverse
//update priority
//concurrency
//look into preemption
pub fn testing() {
    
    let tasks = vec![
        Task { id: 1, rank: 1, state: 0 },
        //Task { id: 2, rank: 2, state: 0 },
        //Task { id: 3, rank: 3, state: 0 },
        //Task { id: 4, rank: 4, state: 0 },
        //Task { id: 5, rank: 5, state: 0 },
    ]; 
    let avl = AvlTree::new(tasks[0].clone());
    println!("{:#?}", avl);
    
}
