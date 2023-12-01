use super::linklist::*;
use super::Task;
use std::cmp::Ordering;
use std::ptr::write;
use std::sync::{Arc, Mutex};

// An AVL tree is a self-balancing binary search tree. It ensures that the height
// difference between the left and right subtrees of any node (the balance factor)
// does not exceed 1. This balancing property helps maintain the tree's height in
// O(log n), where n is the number of nodes.
// The value will be either a single task or a linked list

// enum to allow either a Task type or LinkList type

#[derive(Debug, Clone)]
pub struct AvlTree {
    val: Option<LinkList>,
    height: i32,
    left: Option<Arc<Mutex<AvlTree>>>,
    right: Option<Arc<Mutex<AvlTree>>>,
}

impl AvlTree {
    pub fn new() -> Self {
        AvlTree {
            val: None,
            height: 1,
            left: None,
            right: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.val.is_none() && self.height == 1
    }

    // searches by rank to easily find the node that a task belongs to
    // this function will return the root node that holds the rank
    pub fn search_by_task(&self, target: &Task) -> Option<AvlTree> {
        if self.is_empty() {
            return None;
        }
        if let Some(ll) = &self.val {
            if ll.is_empty() {
                return None;
            }
            //base case if there is a single value
            let cur_node = ll.get_head().unwrap().borrow().clone();
            let cur_node_rank = cur_node.get_rank();
            if target.get_rank() == cur_node_rank {
                return Some(self.clone());
            } else if target.get_rank() < cur_node_rank {
                if let Some(left) = &self.left {
                    return left.lock().unwrap().search_by_task(target);
                }
            } else if target.get_rank() > cur_node_rank {
                if let Some(right) = &self.right {
                    return right.lock().unwrap().search_by_task(target);
                }
            }
        }
        None
    }

    // TODO create the real public search task
    // issue when adding the new value it seemed to delete the previous one

    //traverse through and display the path to value
    pub fn insert(&mut self, new_task: Task) {
        if self.is_empty() {
            let mut ll = LinkList::new();
            ll.push_back(new_task);
            self.val = Some(ll);
        } else {
            self.r_insert(new_task);
        }
        self.update_height();
    }
    // Searches node to insert into
    fn r_insert(&mut self, new_val: Task) {
        match self.val {
            Some(ref mut ll) => {
                let cur_node = ll.get_head().unwrap().borrow().clone();
                match new_val.get_rank().cmp(&cur_node.get_rank()) {
                    Ordering::Equal => {
                        ll.push_back(new_val);
                    }
                    Ordering::Greater => {
                        if let Some(right) = &mut self.right {
                            right.lock().unwrap().r_insert(new_val);
                        } else {
                            let new_tree = AvlTree::new();
                            self.right = Some(Arc::new(Mutex::new(new_tree)));
                            self.right
                                .as_ref()
                                .unwrap()
                                .lock()
                                .unwrap()
                                .r_insert(new_val);
                        }
                    }
                    Ordering::Less => {
                        if let Some(left) = &mut self.left {
                            left.lock().unwrap().r_insert(new_val);
                        } else {
                            let new_tree = AvlTree::new();
                            self.left = Some(Arc::new(Mutex::new(new_tree)));
                            self.left
                                .as_ref()
                                .unwrap()
                                .lock()
                                .unwrap()
                                .r_insert(new_val);
                        }
                    }
                }
            }
            None => {
                let mut new_ll = LinkList::new();
                new_ll.push_back(new_val);
                self.val = Some(new_ll);
                self.height = 1;
            }
        }
        self.update_height();
    }

    // will delete a task within the tree
    fn delete_by_task(&mut self, target: &Task) -> Option<Task> {
        // search task
        if let Some(leaf) = self.search_by_task(target) {
            if let Some(mut ll) = leaf.val {
                let retrieved = ll.delete_task(target);
                self.val = Some(ll);
                return retrieved;
            }
        }
        None
    }

    //update height
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

    // left rotation left imbalance
    /*          root -> right-> right      root-> right -> left
     *           6         7                   6            8
     *             \      / \                    \        /  \
     *              7 -> 6   8                   8  ->  6    7
     *               \                           /
     *                8                         7
     *
     * */
    //fn left_rotation(&mut self)
    //root -> right
    // root-> right -> left
    // right grandchild val
    // left child val
    // root -> right -> right
    // update_height()

    // balance()

    // right rotation left imbalance
    /*          root -> left-> left      root-> left -> Right
     *           5     4                6         4
     *          /     / \              /        /  \
     *         4 ->  3   5            4    ->  5    6
     *        /                        \
     *      3                           5
     *
     * */
    //fn right_rotation(&mut self)
    //root -> left
    // root-> left -> right
    // right grandchild val
    // left child val
    // root -> left -> left
    // update_height()
    //balance()

    //blance factor function is the difference between the height
    //fn balance_factor(&self) -> i32

    // balance the tree after inserting
    //fn balance(&mut self) {
    //left
    //LR
    //left tree is lower then the right tree left rotation on left child
    //LL
    //left tree higher then the right subtee right_rotation
    //right
    //RL
    //right tree higher then the left subtee left_rotation
    //RR
    //right tree is lower then the left tree right rotation on right child

    // Function to check if the AVL tree is balanced
    /*fn is_avl_balanced(&self) -> bool {
        let left_height = self
            .left
            .as_ref()
            .map_or(0, |node| node.lock().unwrap().height);
        let right_height = self
            .right
            .as_ref()
            .map_or(0, |node| node.lock().unwrap().height);

        (left_height - right_height).abs() <= 1
    }*/

    //balance
    //Delete
    //No subtree case
    //One Subtree case
    //Two subtree case
    //traverse
    //update priority
    //concurrency
    //look into preemption */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_tasks() {
        let mut avl_tree = AvlTree::new();

        let tasks = vec![
            Task::new(1, 5, 0),
            Task::new(2, 6, 0),
            Task::new(3, 3, 0),
            Task::new(4, 4, 0),
            Task::new(5, 5, 0),
        ];

        for task in &tasks {
            avl_tree.insert(task.clone());
        }

        assert_eq!(avl_tree.height, 3);
        match avl_tree.val {
            Some(ref ll) => assert_eq!(ll.len(), 2),
            None => unreachable!(),
        }

        let tasks2 = vec![
            Task::new(15, 9, 0),
            Task::new(16, 5, 0),
            Task::new(14, 4, 0),
            Task::new(12, 8, 0),
            Task::new(8, 6, 0),
            Task::new(11, 3, 0),
            Task::new(6, 10, 0),
            Task::new(8, 3, 0),
            Task::new(7, 7, 0),
            Task::new(10, 10, 0),
            Task::new(10, 5, 0),
        ];

        for task in &tasks2 {
            avl_tree.insert(task.clone());
        }

        assert_eq!(avl_tree.height, 5);
        match avl_tree.val {
            Some(ref ll) => assert_eq!(ll.len(), 4),
            None => unreachable!(),
        }
        let search_result = avl_tree.search_by_task(&tasks[2]);
        assert!(search_result.is_some());
        if let Some(node) = search_result {
            if let Some(ll) = &node.val {
                assert_eq!(ll.len(), 3);
            } else {
                unreachable!()
            }
        }
    }

    #[test]
    fn test_search_by_task() {
        let mut avl_tree = AvlTree::new();

        let tasks = vec![
            Task::new(1, 5, 0),
            Task::new(2, 6, 0),
            Task::new(3, 3, 0),
            Task::new(4, 4, 0),
            Task::new(5, 5, 0),
        ];

        for task in &tasks {
            avl_tree.insert(task.clone());
        }
        // Test search for existing task
        let search_result = avl_tree.search_by_task(&tasks[2]);
        assert!(search_result.is_some());
        if let Some(node) = search_result {
            match node.val {
                Some(ll) => {
                    // Check the rank of the retrieved task
                    let retrieved_task = ll.get_head().unwrap();
                    assert_eq!(retrieved_task.borrow().get_rank(), tasks[2].get_rank());
                }
                None => unreachable!(),
            }
        }

        // Test search for non-existing task
        let non_existing_task = Task::new(10, 10, 0);
        let non_existing_search_result = avl_tree.search_by_task(&non_existing_task);
        assert!(non_existing_search_result.is_none());

        let tasks2 = vec![
            Task::new(15, 9, 0),
            Task::new(16, 5, 0),
            Task::new(14, 4, 0),
            Task::new(12, 8, 0),
            Task::new(8, 6, 0),
            Task::new(11, 3, 0),
            Task::new(6, 10, 0),
            Task::new(8, 3, 0),
            Task::new(7, 7, 0),
            Task::new(10, 10, 0),
            Task::new(10, 5, 0),
        ];

        for task in &tasks2 {
            avl_tree.insert(task.clone());
        }

        let new_existing_task = Task::new(10, 10, 0);
        let new_existing_search_result = avl_tree.search_by_task(&new_existing_task);
        assert!(new_existing_search_result.is_some());
    }

    #[test]
    fn test_delete_by_task() {
        // Test deleting from a non-empty tree
        let mut avl_tree = AvlTree::new();

        let tasks = vec![
            Task::new(1, 5, 0),
            Task::new(2, 6, 0),
            Task::new(3, 3, 0),
            Task::new(4, 4, 0),
            Task::new(5, 5, 0),
        ];

        for task in &tasks {
            avl_tree.insert(task.clone());
        }

        // Delete a task that exists
        avl_tree.delete_by_task(&tasks[0]);
        assert_eq!(avl_tree.height, 3);
        match avl_tree.val {
            Some(ref ll) => {
                assert_eq!(ll.len(), 1);
                assert_eq!(ll.get_head().unwrap().borrow().get_id(), tasks[4].get_id());
            }
            None => unreachable!(),
        }

        // Delete a task that doesn't exist
        let non_existent_task = Task::new(100, 6, 0);
        avl_tree.delete_by_task(&non_existent_task);
        assert_eq!(avl_tree.height, 3); // Height should remain the same
        match avl_tree.val {
            Some(ref ll) => {
                assert_eq!(ll.len(), 1); // List size should remain the same
            }
            None => unreachable!(),
        }

        // Test deleting from an empty tree
        let mut empty_tree = AvlTree::new();
        empty_tree.delete_by_task(&tasks[0]); // Delete from an empty tree
        assert_eq!(empty_tree.height, 1); // Height should remain 0
        assert!(empty_tree.is_empty()); // Tree should remain empty
    }
}
