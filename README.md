# Preemptive Task Scheduler

**Project Keywords:** Systems Programming, Data Structure Development, Preemption, Priority Queue, Doubly Linked List, Concurrent AVL Tree

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

### Concurrent AVL Tree

The concurrent AVL tree plays a central role in managing tasks within the scheduler. As a self-balancing data structure, it ensures that the heights and depths of each subtree remain balanced, facilitating efficient task management. Notable functionalities include

- [ ] Insert
- [ ] Delete
- [ ] Get Highest Priority 
- [ ] Update Priority
- [ ] Find Task
- [ ] Traverse
- [ ] Is Empty

**Note:** To address concurrency challenges such as race conditions and deadlocks, safeguards and proper synchronization mechanisms will be incorporated into the AVL tree's design and implementation.

## Scheduling Algorithm

### Purpose
The scheduling algorithm, to be developed after the data structures, is designed for preemptive task management, enabling the prompt execution of high-priority tasks while accommodating dynamic priority changes.

### Benefits
The preemptive approach allows the scheduler to interrupt and execute tasks, providing superior control over task prioritization.

### Challenges
The primary challenge of this preemptive scheduler is the unpredictability of task execution times. Furthermore, lower-priority tasks may struggle to be executed if high-priority tasks frequently preempt them.

## Project Goals and Milestones

- [x] Develop the Doubly Linked List data structure
- [ ] Create the Concurrent AVL Tree data structure
- [ ] Implement core scheduling algorithm
- [ ] Address and test concurrency issues
- [ ] Optimize and fine-tune task scheduling and execution
- [ ] Document and test the project thoroughly


