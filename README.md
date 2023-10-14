# Real-Time Scheduling:

Low Level Practice

## Scheduling Algorithm:
Real-time scheduling aims to meet strict timing requirements for tasks. 
Algorithms like Rate Monotonic Scheduling (RMS) and Earliest Deadline First (EDF) are used. 
RMS assigns priorities based on task periods, while EDF schedules tasks based on their deadlines.

## Data Structure: 
Priority queues are commonly used in real-time scheduling to efficiently manage the order of task execution. 
Priority queues allow tasks with the earliest deadline (in the case of EDF) or the highest priority (in the case of RMS) to be executed first. 
A concurrent doubly linked list priority queue could be suitable for EDF, as it allows for efficient insertion and removal of tasks with changing deadlines. 
However, synchronization and concurrent access management are crucial due to the real-time nature of the scheduling.
