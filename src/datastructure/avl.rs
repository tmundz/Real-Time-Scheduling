use super::Task;
use super::linklist;
use super::linklist::*;
use std::sync::{Arc, Mutex};
use std::cmp::Ordering;

// An AVL tree is a self-balancing binary search tree. It ensures that the height
// difference between the left and right subtrees of any node (the balance factor)
// does not exceed 1. This balancing property helps maintain the tree's height in
// O(log n), where n is the number of nodes.
// The value will be either a single task or a linked list

// enum to allow either a Task type or LinkList type

#[derive(Debug, Clone)]
pub enum TaskorLink {
    STask(Task),
    Link(LinkList),
}

#[derive(Debug, Clone)]
pub struct AvlTree {
    val: Option<TaskorLink>,
    height: i32,
    left: Option<Arc<Mutex<AvlTree>>>,
    right: Option<Arc<Mutex<AvlTree>>>,
}

impl AvlTree {
    fn new(task: Task) -> Self {
        AvlTree {
            val: Some(TaskorLink::STask(task)), 
            height: 1,
            left: None,
            right: None,
        }
    }

    // recursive insert 
    pub fn insert(&mut self, new_val: Task) {
        match &mut self.val {
            //check if there is a task value
            Some(TaskorLink::STask(cur_task)) => {
                //base case for single tasks in a leaf
                //if ranks don't match insert into a left or right leaf
                match cur_task.rank.cmp(&new_val.rank) {
                    Ordering::Equal =>  {
                        let mut ll = linklist::LinkList::new();
                        ll.push_back(cur_task.clone());
                        ll.push_back(new_val);
                        self.val = Some(TaskorLink::Link(ll));

                    }
                    Ordering::Greater => {
                        if let Some(right) = &mut self.right {
                            let mut right_leaf = right.lock().unwrap();
                            right_leaf.insert(new_val);
                        }else {
                            let new_node = AvlTree::new(new_val);
                            self.right = Some(Arc::new(Mutex::new(new_node)));
                        }
                    }
                    Ordering::Less => {
                        if let Some(left) = &mut self.left {
                            let mut left_leaf = left.lock().unwrap();
                            left_leaf.insert(new_val);
                        } else {
                            let new_node = AvlTree::new(new_val);
                            self.left = Some(Arc::new(Mutex::new(new_node)));
                        }
                    }
                }
            }
            
            // if leaf node already contains a Doubly
            // linklist check and push to back
            // else insert into a right or left node
            Some(TaskorLink::Link(ll)) => {
                let cur_node = ll.get_head().unwrap().borrow().clone();
                match cur_node.rank.cmp(&new_val.rank) {
                    Ordering::Equal => {
                        ll.push_back(new_val);
                    }
                    Ordering::Greater => {

                        if let Some(right) = &mut self.right {
                            let mut right_leaf = right.lock().unwrap();
                            right_leaf.insert(new_val);
                        }
                    }

                    Ordering::Less => {
                        
                        if let Some(left) = &mut self.left {
                            let mut left_leaf = left.lock().unwrap();
                            left_leaf.insert(new_val);
                        } 
                    }
                }
            }
            // If rank does not exist create a new leaf
            None => {
                self.val = Some(TaskorLink::STask(new_val));
                self.left = None;
                self.right = None;
                self.height = 1;
            }
        }
        //update height 
        //update balance
    }

    //update height function
    fn update_height(&mut self) {
        let left_height = self.left.as_ref().map(|node| node.lock().unwrap().height).unwrap_or(0);
        let right_height = self.right.as_ref().map(|node| node.lock().unwrap().height).unwrap_or(0);

        self.height = 1 + std::cmp::max(left_height, right_height);
    }

    fn display(&self, indent: String) {
        match &self.val {
            Some(TaskorLink::STask(task)) => {
                println!("{}Task: id={}, rank={}, state={}", indent, task.id, task.rank, task.state);
            }
            Some(TaskorLink::Link(link_list)) => {
                println!("{}Linked List:", indent);
                link_list.display(&format!("{}  ", indent));
            }
            None => {
                println!("{}Empty", indent);
            }
        }

        if let Some(left) = &self.left {
            left.lock().unwrap().display(format!("{}L: ", indent));
        }

        if let Some(right) = &self.right {
            right.lock().unwrap().display(format!("{}R: ", indent));
        }
    }
            //fn update
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
        Task { id: 2, rank: 2, state: 0 },
        Task { id: 3, rank: 3, state: 0 },
        Task { id: 8, rank: 4, state: 0 },
        Task { id: 7, rank: 4, state: 0 },
        Task { id: 6, rank: 4, state: 0 },
        Task { id: 4, rank: 3, state: 0 },
        Task { id: 5, rank: 5, state: 0 },
        Task { id: 7, rank: 0, state: 0 },
        Task { id: 10, rank: 5, state: 0 },
        Task { id: 9, rank: 1, state: 0 },
        Task { id: 11, rank: 2, state: 0 },
        Task { id: 12, rank: 6, state: 0 },
        Task { id: 13, rank: 10, state: 0 },
        Task { id: 14, rank: 10, state: 0 },
    ]; 
    let mut avl = AvlTree::new(tasks[2].clone());
    avl.insert(tasks[1].clone());
    avl.insert(tasks[9].clone());
    avl.insert(tasks[10].clone());
    avl.insert(tasks[11].clone());
    avl.insert(tasks[12].clone());
    avl.insert(tasks[13].clone());
    avl.insert(tasks[0].clone());
    avl.insert(tasks[8].clone());
    avl.insert(tasks[7].clone());
    avl.insert(tasks[3].clone());
    avl.insert(tasks[4].clone());
    avl.insert(tasks[5].clone());
    avl.insert(tasks[6].clone());

    avl.display(' '.to_string()); 
    //println!("{:#?}",avl);
}

#[cfg(test)]
mod test {
    use super::AvlTree;
    use super::Task;
    fn insert_test() {
        
        println!("hello");
    }
}
