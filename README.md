# Preemptive scheduler:

Systems Programming, data structure development and implemintation

## Scheduling Algorithm:
Our scheduling algorithm is based on preemption, allowing higher-priority tasks to preempt lower-priority tasks. This approach ensures that critical tasks are executed promptly. Task priorities are subject to dynamic changes, and we employ an AVL tree to maintain an organized order efficiently.

## Data Structure: 
My goal is to design a priority queue using a Linked list to help manage the main queue (FIFO), while
there will be an AVL tree used to update the list as priorities change.
