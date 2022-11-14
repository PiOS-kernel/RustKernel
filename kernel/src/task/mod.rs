use core::ptr;
use alloc::boxed::Box;
use core::marker::Sync;
use crate::{utility::memcpy, mutex::Mutex};

//file to be reviewed, probably need to split it into modules, probably need to address some details

//defined type 
type TcbBlock = Option<Box<TaskTCB>>;           //used as a reference to a Task_TCB 
const STACK_SIZE: usize = 1024;                         //size of the stack for every task

//global variables
pub const MAX_PRIORITY: u8 = 10;                    //max priority and size of the priority queues array
static mut RUNNING: TcbBlock = None;                       //variable for the running task.. best way to store it?

//definition of the Task Control Block 

pub struct TaskTCB {
    next: TcbBlock,                     //reference to the next Task_TCB
    pub priority: u8,                       //priority of the task
    stp: usize,                         //stack pointer
    stack: [u8; STACK_SIZE],            //stack associated to the task
}

impl TaskTCB {

    //constructor for a TaskTCB that return an instance of a TaskTCB 
    //with the associating the parameters to the corresponding fields
    pub fn new (n: TcbBlock, p: u8) -> Self{    
        Self { next: n, priority: p, stp: 0, stack: [0; STACK_SIZE] }
    }

    // utility method to push values onto the task's stack
    pub fn stack_push(&mut self, ptr: *mut u8, size: usize) {
        // Check whether there is room left on the stack
        if self.stp > STACK_SIZE - size {
            panic!();
        }

        // The data is stored onto the stack and the stack pointer
        // is incremented.
        unsafe {
            let base = (&mut self.stack) as *mut u8;
            memcpy(ptr, base.add(self.stp), size);
        }
        self.stp += size;
    }

    // this method allows to access the task's stack pointer as a raw memory
    // address
    pub fn get_stp(&self) -> *mut u8 {
        unsafe{
            let base = &self.stack as *const u8 as *mut u8;
            base.add(self.stp)
        }
    }
}

/*
This struct is simply a wrapper to the `Queue` struct,
it uses a mutex to encapsulate the queue.
It is necessary because the queue will be declared as 
static and because of rust rules it will be necessary 
to get a `&mut` reference out of a `&` reference.
*/
pub struct LockedQueue {
    mux: Mutex<Queue>
}

impl LockedQueue {
    pub const fn new() -> Self {
        Self {mux: Mutex::new(Queue::new())}
    }
    pub fn enqueue(&self, block: TaskTCB) {
        let mut queue = self.mux.lock();
        queue.enqueue(block);
    }
    pub fn dequeue(&self) -> Option<TaskTCB> {
        let mut queue = self.mux.lock();
        queue.dequeue()
    }
}

//struct of a queue of TaskTCB
pub struct Queue {
    head: TcbBlock,
    tail: *mut TaskTCB,
}

impl Queue {
    
    //initialize the queue with both head and tail None
    pub const fn new () -> Self{
        Self {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    //return true if the queue is empty
    pub fn empty (& self) -> bool {
        self.head.is_none()
    }

    //enqueue a TaskTCB at the end of the queue
    pub fn enqueue (&mut self, block: TaskTCB) {
        let mut new_tail = Box::new(block);       //create a new Box<TaskTCB> pointing to the new element to add
        let tail_ptr: *mut _ = &mut *new_tail;    //create raw pointer to the new element just created

        if self.empty(){      //if the queue is empty add the element in the head
            self.head = Some(new_tail);
        } else {
            unsafe {                                            //if it is not empty add the elemente in the tail.next
                (*self.tail).next = Some(new_tail);
            }
        }
        self.tail = tail_ptr;                                   //update the tail to the new end of the queue
    }

    //dequque the first element of the queue
    pub fn dequeue (&mut self) -> Option<TaskTCB> {
        if let Some(mut old_head) = self.head.take() {          
            match old_head.next.take() {                       
                Some(task_tcb) => {                             
                    self.head = Some(task_tcb);
                }
                None => self.tail = ptr::null_mut(),            //shift the head to the current head.next and update the tail if   
            }                                                   //it is the last element
            Some(*old_head)              //return the popped element
        } else {
            None                        //return None if the queue was already empty
        }
    }

    // scheduling function... rather simple for now considering only one queue and never ending tasks
    pub fn schedule(q : &mut Queue) {    

        if !q.empty() {
            let task = q.dequeue();             //take the first tasks in the queue and place it in last 
            //context_switch()                  //still need to implement the context switch part 
            q.enqueue(task.unwrap());
        } 
    }

}

unsafe impl Sync for Queue {}


/*  just a test

fn main(){

    let mut test_queue: Queue;
    test_queue = Queue::new();
    let task1: TaskTCB;
    task1 = TaskTCB::new(None,1,None);
    let task2: TaskTCB;
    task2 = TaskTCB::new(None,2,None);
    let task3: TaskTCB;
    task3 = TaskTCB::new(None,3,None);
    let task4: TaskTCB;
    task4 = TaskTCB::new(None,4,None);
    
    test_queue.enqueue(task1);
    test_queue.enqueue(task2);
    //test_queue.print();
    test_queue.enqueue(task3);
    test_queue.enqueue(task4);
    //test_queue.print();
    test_queue.dequeue();
    test_queue.dequeue();
    //test_queue.print();
    test_queue.dequeue();
    test_queue.dequeue();
    test_queue.dequeue();

}*/