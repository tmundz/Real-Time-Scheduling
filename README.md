# Multi-Level Feedback Queue (MLFQ) Scheduler

**Project Keywords:** Systems Programming, Data Structure Development, Preemption, Priority Queue, Doubly Linked List, concurrency, AVL Tree

## Introduction

This project aims to develop a preemptive task scheduler designed for systems programming. The scheduler incorporates a flexible priority-based preemptive algorithm that allows higher-priority tasks to interrupt a task and then be executed, ensuring the timely execution of critical tasks. Task priorities are subject to dynamic changes, offering granular control over task execution.

## Data Structures

### Doubly Linked List

The foundation of the scheduler's task queue is a doubly linked list. Using this data structure simplifies the reordering of tasks as priorities change and facilitates task order maintenance for monitoring purposes. Key functionalities include:

- [x] Insert
- [x] Delete
- [x] Traversing
- [x] Is Empty
- [x] Find Task
- [x] Testing

### Concurrent AVL Tree

The concurrent AVL tree plays a central role in managing tasks within the scheduler. As a self-balancing data structure, it ensures that the heights and depths of each subtree remain balanced, facilitating efficient task management. Notable functionalities include

- [x] Is Empty
- [x] Insert
- [ ] Balance
- [ ] Delete
- [ ] Get Highest Priority
- [ ] Update Priority
- [ ] Find Task
- [ ] Traverse
- [ ] Testing

**Note:** To address concurrency challenges such as race conditions and deadlocks, safeguards and proper synchronization mechanisms will be incorporated into the AVL tree's design and implementation.

#### Current Issues

I am struggling with the implementation of the rotations, making it challenging to continue with the scheduling algorithm. Therefore, I decided to start with a basic BST implementation, which means it works much the same, except there will be no balance features. I will implement these features later.

### Use

The AVL tree will insert nodes based on the task's rank, where the rank is represented as an i32 integer. The higher the number, the higher the priority. Each node will contain either a single task or a doubly linked list of tasks.

                                  (rank: 5, task: task3)
                                  /                     \
    (rank: 3, task: [task1 <-> task4 <-> task6])         (rank:6, task: [task2 <-> task5])

## Scheduling Algorithm

### Purpose

The scheduling algorithm, to be developed after the data structures, is designed for preemptive task management, enabling the prompt execution of high-priority tasks while accommodating dynamic priority changes.

### Benefits

The preemptive approach allows the scheduler to interrupt and execute tasks, providing superior control over task prioritization.

### Challenges

The primary challenge of this preemptive scheduler is the unpredictability of task execution times. Furthermore, lower-priority tasks may struggle to be executed if high-priority tasks frequently preempt them.

#### Possible Solutions

To deal with the stagnation challenge it would make sense to include an aging function. Another solution to have better control of the execution could potentially be running a round robin in each leaf node list.

## Project Goals and Milestones

- [x] Develop the Doubly Linked List data structure
- [ ] Create the Concurrent AVL Tree data structure
- [ ] Implement core scheduling algorithm
- [ ] Address and test concurrency issues
- [ ] Optimize and fine-tune task scheduling and execution
- [ ] Document and test the project thoroughly

#### Evolution of Project

Priority Queue -> Preemptive Priority Queue -> Multi-Level Feedback Queue

The reason for the for the changes in Scheduler was at first wanting to make the scheduler more complex by allowing preemption. Then during the implementation of the AVL Tree it made sense to me to include not just single tasks in the nodes for me I thought it was best to store the doubly linked lists into the nodes for each rank, since in this situation size of the program is not a concern.
