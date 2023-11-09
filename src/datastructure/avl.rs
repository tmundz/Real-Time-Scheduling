use super::linklist;
use super::linklist::*;
use super::Task;
use std::cmp::Ordering;
use std::sync::{Arc, Mutex};

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

    // checks if the tree is empty
    pub fn is_empty(&self) -> bool {
        self.val.is_none()
    }

    pub fn insert(&mut self, new_val: Task) {
        if self.is_empty() {
            self.val = Some(TaskorLink::STask(new_val));
        } else {
            self.r_insert(new_val);
        }
    }
    // recursive insert

    fn r_insert(&mut self, new_val: Task) {
        match &mut self.val {
            Some(TaskorLink::STask(cur_task)) => match cur_task.rank.cmp(&new_val.rank) {
                Ordering::Equal => {
                    let mut ll = linklist::LinkList::new();
                    ll.push_back(cur_task.clone());
                    ll.push_back(new_val);
                    self.val = Some(TaskorLink::Link(ll));
                }
                Ordering::Greater => {
                    if let Some(right) = &mut self.right {
                        let mut right_leaf = right.lock().unwrap();
                        right_leaf.insert(new_val);
                    } else {
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
            },
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
            None => {
                self.val = Some(TaskorLink::STask(new_val));
                self.left = None;
                self.right = None;
                self.height = 1;
            }
        }
        self.balance();
    }

    // left rotation left imbalance
    /*          root -> right-> right      root-> right -> left
     *           6         7                   6            8
     *             \      / \                    \        /  \
     *              7 -> 6   8                   8  ->  6    7
     *               \                           /
     *                8                         7
     *
     * */
    fn left_rotation(&mut self) {
        //root -> right
        if let Some(mut new_root) = self.right.take() {
            // root-> right -> left
            if let Some(new_right) = new_root.lock().unwrap().left.take() {
                // right grandchild val
                let new_right_data = new_right.lock().unwrap().val.take();
                // left child val
                let new_root_data = new_root.lock().unwrap().val.take();

                let new_left = AvlTree {
                    val: self.val.take(),
                    height: self.height,
                    left: self.left.take(),
                    right: None,
                };

                self.val = new_root_data;
                self.left = Some(Arc::new(Mutex::new(new_left)));
                self.right = new_root.lock().unwrap().left.take();

                // root -> right -> right
            } else {
                let new_root_data = new_root.lock().unwrap().val.take();

                let new_left = AvlTree {
                    val: self.val.take(),
                    height: self.height,
                    left: self.left.take(),
                    right: None,
                };

                self.val = new_root_data;
                self.left = Some(Arc::new(Mutex::new(new_left)));
                self.right = new_root.lock().unwrap().right.take();
            }
        }
        // update height
        self.update_height();
        self.balance();
    }

    // right rotation left imbalance
    /*          root -> left-> left      root-> left -> Right
     *           5     4                6         4
     *          /     / \              /        /  \
     *         4 ->  3   5            4    ->  5    6
     *        /                        \
     *      3                           5
     *
     * */
    fn right_rotation(&mut self) {
        //root -> left
        if let Some(mut new_root) = self.left.take() {
            // root-> left -> right
            if let Some(new_left) = new_root.lock().unwrap().right.take() {
                // right grandchild val
                let new_left_data = new_left.lock().unwrap().val.take();
                // left child val
                let new_root_data = new_root.lock().unwrap().val.take();

                let new_right = AvlTree {
                    val: self.val.take(),
                    height: self.height,
                    left: None,
                    right: self.right.take(),
                };

                self.val = new_root_data;
                self.left = new_root.lock().unwrap().left.take();
                self.right = Some(Arc::new(Mutex::new(new_right)));

            // root -> left -> left
            } else {
                let new_root_data = new_root.lock().unwrap().val.take();
                let new_right = AvlTree {
                    val: self.val.take(),
                    height: self.height,
                    left: None,
                    right: self.right.take(),
                };

                self.val = new_root_data;
                self.left = new_root.lock().unwrap().left.take();
                self.right = Some(Arc::new(Mutex::new(new_right)));
            }
        }
        // update height
        self.update_height();
        self.balance();
    }

    //blance factor function is the difference between the height
    fn balance_factor(&self) -> i32 {
        let left_height = self
            .left
            .as_ref()
            .map(|node| node.lock().unwrap().height)
            .unwrap_or(0);
        let right_height = self
            .right
            .as_ref()
            .map(|node| node.lock().unwrap().height)
            .unwrap_or(0);
        left_height - right_height
    }

    //update height function
    fn update_height(&mut self) {
        let left_height = self
            .left
            .as_ref()
            .map(|node| node.lock().unwrap().height)
            .unwrap_or(0);
        let right_height = self
            .right
            .as_ref()
            .map(|node| node.lock().unwrap().height)
            .unwrap_or(0);

        self.height = 1 + std::cmp::max(left_height, right_height);

        // Update height after rotations
        self.left
            .as_ref()
            .map(|node| node.lock().unwrap().update_height());
        self.right
            .as_ref()
            .map(|node| node.lock().unwrap().update_height());
    }
    // balance the tree after inserting
    fn balance(&mut self) {
        self.update_height();

        let balance_factor = self.balance_factor();
        //left
        if balance_factor > 1 {
            let left_child_bal = self
                .left
                .as_ref()
                .map_or(0, |node| node.lock().unwrap().balance_factor());

            //LR
            //left tree is lower then the right tree left rotation on left child
            if left_child_bal < 0 {
                self.left_rotation();
            }
            //LL
            //left tree higher then the right subtee right_rotation
            self.right_rotation();
        //right
        } else if balance_factor < -1 {
            let right_child_bal = self
                .right
                .as_ref()
                .map_or(0, |node| node.lock().unwrap().balance_factor());
            //RL
            //right tree higher then the left subtee left_rotation
            if right_child_bal > 0 {
                self.right_rotation();
            }
            //RR
            //right tree is lower then the left tree right rotation on right child
            self.left_rotation();
        }
        self.update_height();
        self.left
            .as_ref()
            .map(|node| node.lock().unwrap().update_height());
        self.right
            .as_ref()
            .map(|node| node.lock().unwrap().update_height());
    }

    // Function to check if the AVL tree is balanced
    fn is_avl_balanced(&self) -> bool {
        let left_height = self
            .left
            .as_ref()
            .map_or(0, |node| node.lock().unwrap().height);
        let right_height = self
            .right
            .as_ref()
            .map_or(0, |node| node.lock().unwrap().height);

        (left_height - right_height).abs() <= 1
    }

    fn display(&self, indent: String) {
        match &self.val {
            Some(TaskorLink::STask(task)) => {
                println!(
                    "{}Task: id={}, rank={}, state={}",
                    indent, task.id, task.rank, task.state
                );
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_tasks() {
        // Create an AVL tree
        let mut avl_tree = AvlTree::new(Task::new(0, 0, 0));

        // Define a vector of tasks
        let tasks = vec![
            Task::new(1, 1, 0),
            Task::new(2, 2, 0),
            Task::new(3, 3, 0),
            Task::new(4, 4, 0),
            Task::new(5, 5, 0),
        ];

        // Insert tasks into the AVL tree
        for task in tasks.iter() {
            avl_tree.insert(task.clone());
        }

        // Verify the structure and content of the AVL tree
        assert_eq!(avl_tree.height, 4);
        assert!(avl_tree.is_avl_balanced());

        // Add more assertions to check the content and structure of the AVL tree
    }

    #[test]
    fn test_left_rotation() {
        let mut avl_tree = AvlTree::new(Task::new(5, 5, 0));
        avl_tree.insert(Task::new(4, 4, 0));
        avl_tree.insert(Task::new(3, 3, 0));

        // Verify left rotation
        assert_eq!(avl_tree.height, 3);
        assert!(avl_tree.is_avl_balanced());
    }

    #[test]
    fn test_right_rotation() {
        let mut avl_tree = AvlTree::new(Task::new(3, 3, 0));
        avl_tree.insert(Task::new(4, 4, 0));
        avl_tree.insert(Task::new(5, 5, 0));

        // Verify right rotation
        assert_eq!(avl_tree.height, 3);
        assert!(avl_tree.is_avl_balanced());
    }

    #[test]
    fn test_left_right_rotation() {
        let mut avl_tree = AvlTree::new(Task::new(5, 5, 0));
        avl_tree.insert(Task::new(3, 3, 0));
        avl_tree.insert(Task::new(4, 4, 0));

        // Verify left-right rotation
        assert_eq!(avl_tree.height, 3);
        assert!(avl_tree.is_avl_balanced());
    }

    #[test]
    fn test_right_left_rotation() {
        let mut avl_tree = AvlTree::new(Task::new(3, 3, 0));
        avl_tree.insert(Task::new(5, 5, 0));
        avl_tree.insert(Task::new(4, 4, 0));

        // Verify right-left rotation
        assert_eq!(avl_tree.height, 3);
        assert!(avl_tree.is_avl_balanced());
    }

    // Add more test cases as needed
}
