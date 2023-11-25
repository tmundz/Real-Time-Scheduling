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

    //TODO create a search by task that will first call search by rank

    pub fn search_by_rank(&self, t_rank: i32) -> Option<AvlTree> {
        match &self.val {
            //base case if there is a single value
            Some(TaskorLink::STask(cur_task)) => {
                if t_rank == cur_task.rank {
                    return Some(self.clone());
                } else if t_rank < cur_task.rank {
                    if let Some(left) = &self.left {
                        return left.lock().unwrap().search_by_rank(t_rank);
                    }
                } else if t_rank > cur_task.rank {
                    if let Some(right) = &self.right {
                        return right.lock().unwrap().search_by_rank(t_rank);
                    }
                }
            }
            Some(TaskorLink::Link(ll)) => {
                let cur = ll.get_head().unwrap().borrow().clone();
                let cur_rank = cur.rank;
                if t_rank == cur_rank {
                    return Some(self.clone());
                } else if t_rank < cur_rank {
                    if let Some(left) = &self.left {
                        return left.lock().unwrap().search_by_rank(t_rank);
                    }
                } else if t_rank > cur_rank {
                    if let Some(right) = &self.right {
                        return right.lock().unwrap().search_by_rank(t_rank);
                    }
                }
            }
            None => {
                return None;
            }
        }
        None
    }

    //traverse through and display the path to value
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
            Some(TaskorLink::STask(cur_task)) => match new_val.rank.cmp(&cur_task.rank) {
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
                match new_val.rank.cmp(&cur_node.rank) {
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
        self.update_height();
    }

    fn delete_by_task(&mut self, target: &Task) -> Option<Task> {
        //TODO make sure to add a search by rank call to get the  proper avl tree
        match &mut self.val {
            Some(TaskorLink::Link(ll)) => ll.delete_task(target),
            Some(TaskorLink::STask(cur_task)) => {
                let t = cur_task.clone();
                self.val = None;
                Some(t)
            }
            None => None,
        }
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
        // Create an AVL tree
        let mut avl_tree = AvlTree::new(Task::new(0, 4, 0));

        // Define a vector of tasks
        let tasks = vec![
            Task::new(1, 1, 0),
            Task::new(2, 6, 0),
            Task::new(3, 3, 0),
            Task::new(4, 4, 0),
            Task::new(5, 5, 0),
        ];

        // Insert tasks into the AVL tree
        for task in tasks.iter() {
            avl_tree.insert(task.clone());
        }

        // Verify the structure and content of the AVL tree
        assert_eq!(avl_tree.height, 3);

        // Test search for existing rank
        let search_result = avl_tree.search_by_rank(4);
        assert!(search_result.is_some());
        if let Some(node) = search_result {
            match node.val {
                Some(TaskorLink::STask(_task)) => {
                    unreachable!();
                }
                Some(TaskorLink::Link(ll)) => {
                    assert_eq!(ll.len(), 2);
                }
                None => {
                    unreachable!();
                }
            }
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
        ];
        for task in tasks2.iter() {
            avl_tree.insert(task.clone());
        }

        assert_eq!(avl_tree.height, 5);
        let search_result2 = avl_tree.search_by_rank(4);
        assert!(search_result2.is_some());
        if let Some(node) = search_result2 {
            match node.val {
                Some(TaskorLink::STask(_task)) => {
                    unreachable!();
                }
                Some(TaskorLink::Link(ll)) => {
                    assert_eq!(ll.len(), 3);
                }
                None => {
                    unreachable!();
                }
            }
        }
    }

    #[test]
    fn test_search_by_rank() {
        // Create an AVL tree
        let mut avl_tree = AvlTree::new(Task::new(0, 4, 0));

        // Define a vector of tasks
        let tasks = vec![
            Task::new(1, 1, 0),
            Task::new(2, 6, 0),
            Task::new(3, 3, 0),
            Task::new(4, 4, 0),
            Task::new(8, 4, 0),
            Task::new(5, 5, 0),
        ];

        // Insert tasks into the AVL tree
        for task in tasks.iter() {
            avl_tree.insert(task.clone());
        }

        // Test search for existing rank
        let search_result = avl_tree.search_by_rank(3);
        assert!(search_result.is_some());
        if let Some(node) = search_result {
            match node.val {
                Some(TaskorLink::STask(task)) => {
                    assert!(true);
                    assert_eq!(task.rank, 3);
                    assert_eq!(task.id, 3);
                }
                Some(TaskorLink::Link(_ll)) => {
                    unreachable!();
                }
                None => {
                    unreachable!();
                }
            }
        } else {
            unreachable!();
        }

        // Test search for existing rank
        let search_result = avl_tree.search_by_rank(4);
        assert!(search_result.is_some());
        if let Some(node) = search_result {
            match node.val {
                Some(TaskorLink::STask(_task)) => {
                    unreachable!();
                }
                Some(TaskorLink::Link(ll)) => {
                    let head_rank = ll.get_head().unwrap().borrow().clone();
                    assert_eq!(head_rank.rank, 4);
                    assert_eq!(head_rank.id, 0);
                    let val_2 = ll.search_by_task(tasks[4].clone());
                    assert_eq!(
                        val_2.as_ref().unwrap().borrow().node.borrow().rank.clone(),
                        tasks[4].rank
                    );
                    assert_eq!(
                        val_2.as_ref().unwrap().borrow().node.borrow().id.clone(),
                        tasks[4].id
                    );
                    let val_3 = ll.search_by_task(tasks[3].clone());
                    assert_eq!(
                        val_3.as_ref().unwrap().borrow().node.borrow().rank.clone(),
                        tasks[3].rank
                    );
                    assert_eq!(
                        val_3.as_ref().unwrap().borrow().node.borrow().id.clone(),
                        tasks[3].id
                    );
                }
                None => {
                    unreachable!();
                }
            }
        } else {
            unreachable!();
        }

        // Test search for non-existing rank
        let search_result_non_existing = avl_tree.search_by_rank(8);
        assert!(search_result_non_existing.is_none());
    }

    #[test]
    fn test_delete_by_task() {
        // Create an AVL tree
        let mut avl_tree = AvlTree::new(Task::new(0, 4, 0));

        // Define a vector of tasks
        let tasks = vec![
            Task::new(1, 1, 0),
            Task::new(2, 6, 0),
            Task::new(3, 3, 0),
            Task::new(4, 4, 0),
            Task::new(8, 4, 0),
            Task::new(5, 5, 0),
        ];

        // Insert tasks into the AVL tree
        for task in tasks.iter() {
            avl_tree.insert(task.clone());
        }

        // Test delete existing task
        let task_to_delete = &tasks[2]; // Task with id = 3
        let deleted_task = avl_tree.delete_by_task(task_to_delete);
        assert!(deleted_task.is_some());
        assert_eq!(deleted_task.unwrap().id, task_to_delete.id);

        // Verify that the task is no longer in the AVL tree
        let search_result_after_delete = avl_tree.search_by_rank(task_to_delete.clone());
        assert!(search_result_after_delete.is_none());

        // Test delete non-existing task
        let non_existing_task = Task::new(99, 99, 0); // Assuming this task does not exist
        let deleted_non_existing_task = avl_tree.delete_by_task(&non_existing_task);
        assert!(deleted_non_existing_task.is_none());

        // Verify that the AVL tree structure is still valid after delete operations
        // (You might want to add more assertions based on your AVL tree implementation)
        // For example, check that the AVL tree remains balanced.

        // Test delete last task in the AVL tree
        let last_task_to_delete = &tasks[0]; // Task with id = 1
        let deleted_last_task = avl_tree.delete_by_task(last_task_to_delete);
        assert!(deleted_last_task.is_some());
        assert_eq!(deleted_last_task.unwrap().id, last_task_to_delete.id);

        // Verify that the task is no longer in the AVL tree
        let search_result_after_last_delete =
            avl_tree.search_by_rank(last_task_to_delete.clone().rank);
        assert!(search_result_after_last_delete.is_none());

        // Verify that the AVL tree structure is still valid after delete operations
        // (You might want to add more assertions based on your AVL tree implementation)
        // For example, check that the AVL tree remains balanced.
    }
}
