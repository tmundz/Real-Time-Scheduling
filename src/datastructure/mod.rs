pub mod linklist;
pub mod avl;

/*
 * id to determine a task 
 * rank to determine priority
    ll.push_back(task);
 * state will need to change to a different struct
*/
#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    id: i32,
    rank: i32,
    state: i32 // will change to a task struct
}
