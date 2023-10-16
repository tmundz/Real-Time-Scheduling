# Preemptive scheduler:

Systems Programming, data structure development and implementation

## Scheduling Algorithm:
Our scheduling algorithm is based on preemption, allowing higher-priority tasks to preempt lower-priority tasks. This approach ensures that critical tasks are executed promptly. Task priorities are subject to dynamic changes, and we employ an AVL tree to maintain an organized order efficiently. This algorithm will be asynchronous.

## Data Structure: 
My goal is to design a concurrent priority queue using a normal Doubly Linked list as in this case memory is not an issue and 
it allows for easier traversal since I will no the next and previous node.
The linked list will help manage the main queue (FIFO), while there will be an 
concurrent AVL tree used to update the list as priorities change.
