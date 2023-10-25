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
    height: i32,
    left: Option<Arc<Mutex<AvlTree>>>,
    right: Option<Arc<Mutex<AvlTree>>>,
}

impl AvlTree {
    fn new(task: Task) -> Self {
        AvlTree { 
            val: task, 
            height: 1,
            left: None, 
            right: None, 
        }
        
    }
    //insert
    pub fn insert(&mut self, new_val: Task) {
        match new_val.rank.cmp(&self.val.rank) {
            //if rank is less then
            Ordering::Less => {
                if let Some(left) = &mut self.left {
                    left.lock().unwrap().insert(new_val);
                } else {
                    let new_node = AvlTree::new(new_val);
                    self.left = Some(Arc::new(Mutex::new(new_node)));
                }
            }
            //if rank is greater
            Ordering::Greater => {
                if let Some(right) = &mut self.right {
                    right.lock().unwrap().insert(new_val);
                } else {
                    let new_node = AvlTree::new(new_val);
                    self.right = Some(Arc::new(Mutex::new(new_node)));
                }
            }
            Ordering::Equal => {
                //figure out how to deal with equal compare
                println!("they equal");
            }
            //if ranks are equal
        }
        //update height 
        //update balance
    }
    //balance
    //Delete
    //traverse
    //update priority
    //concurrency
    //look into preemption

}

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
