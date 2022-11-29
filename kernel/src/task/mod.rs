use crate::{mutex::Mutex, utility::memcpy, WAITING_QUEUE};
use alloc::boxed::Box;
use core::borrow::BorrowMut;
use core::marker::Sync;
use core::ptr;

//file to be reviewed, probably need to split it into modules, probably need to address some details

//defined type
type TcbBlock = Option<Box<TaskTCB>>; //used as a reference to a Task_TCB
const STACK_SIZE: usize = 1024; //size of the stack for every task

//global variables
pub const MAX_PRIORITY: u8 = 10; //max priority and size of the priority queues array
pub static mut RUNNING: *mut TaskTCB = ptr::null_mut(); //pointer to the current running task's TaskTCB

//definition of the Task Control Block

pub struct TaskTCB {
    pub stp: usize,              //stack pointer
    pub priority: u8,            //priority of the task
    pub stack: [u8; STACK_SIZE], //stack associated to the task
    pub next: TcbBlock,          //reference to the next Task_TCB
}

impl TaskTCB {
    //constructor for a TaskTCB that return an instance of a TaskTCB
    //with the associating the parameters to the corresponding fields
    pub fn new(n: TcbBlock, p: u8) -> Self {
        Self {
            next: n,
            priority: p,
            stp: 0,
            stack: [0; STACK_SIZE],
        }
    }

    // utility method to push values onto the task's stack
    pub fn stack_push(&mut self, src: *const u8, size: usize) {
        // Check whether there is room left on the stack
        if self.stp > STACK_SIZE - size {
            panic!();
        }

        // The data is stored onto the stack and the stack pointer
        // is incremented.
        unsafe {
            let base = (&mut self.stack[0]) as *mut u8;
            memcpy(src, base.add(self.stp), size);
        }
        self.stp += size;
    }

    // this method allows to access the task's stack pointer as a raw memory
    // address
    pub fn get_stp(&self) -> *mut u8 {
        unsafe {
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
    mux: Mutex<Queue>,
}

impl LockedQueue {
    pub const fn new() -> Self {
        Self {
            mux: Mutex::new(Queue::new()),
        }
    }
    pub fn enqueue(&self, block: Box<TaskTCB>) {
        let mut queue = self.mux.lock();
        queue.enqueue(block);
    }
    pub fn dequeue(&self) -> Option<Box<TaskTCB>> {
        let mut queue = self.mux.lock();
        queue.dequeue()
    }
    pub fn empty(&self) -> bool {
        let queue = self.mux.lock();
        queue.empty()
    }
    pub fn count_tasks(&self) -> usize {
        let mut queue = self.mux.lock();
        queue.count_tasks()
    }
}

//struct of a queue of TaskTCB
pub struct Queue {
    head: TcbBlock,
    tail: *mut TaskTCB,
}

// This type implements the `Iterator` trait, which allows to iterate through
// a Queue through the `for task in queue` construct 
pub struct QueueIterator<'a> {
    next: Option<&'a TaskTCB>,
}

impl Queue {
    //initialize the queue with both head and tail None
    pub const fn new() -> Self {
        Self {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    //return true if the queue is empty
    pub fn empty(&self) -> bool {
        self.head.is_none()
    }

    //returns an iterator
    pub fn iter(&mut self) -> QueueIterator {
        QueueIterator { next: self.head.as_deref() }
    }

    //enqueue a TaskTCB at the end of the queue
    pub fn enqueue(&mut self, mut block: Box<TaskTCB>) {
        let tail_ptr: *mut _ = &mut *block; //create raw pointer to the new element just created

        if self.empty() {
            //if the queue is empty add the element in the head
            self.head = Some(block);
        } else {
            unsafe {
                //if it is not empty add the elemente in the tail.next
                (*self.tail).next = Some(block);
            }
        }
        self.tail = tail_ptr; //update the tail to the new end of the queue
    }

    //dequque the first element of the queue
    pub fn dequeue(&mut self) -> Option<Box<TaskTCB>> {
        if let Some(mut old_head) = self.head.take() {
            match old_head.next.take() {
                Some(task_tcb) => {
                    self.head = Some(task_tcb);
                }
                None => self.tail = ptr::null_mut(), //shift the head to the current head.next and update the tail if
            } //it is the last element
            Some(old_head) //return the popped element
        } else {
            None //return None if the queue was already empty
        }
    }

    //returns the number of tasks currently in the queue
    pub fn count_tasks(&mut self) -> usize {
        let mut count = 0;
        for _ in self.iter() {
            count += 1;
        }
        count 
    } 
}
 
// scheduling function for now considering only one queue and never ending tasks
#[no_mangle]
pub unsafe fn schedule() -> *mut TaskTCB {
    if !WAITING_QUEUE.empty() {
        //take the first tasks in the queue
        let task = WAITING_QUEUE.dequeue();

        //set RUNNING accordingly
        match task {
            Some(mut task) => {
                // Casting a read only reference to a mutable pointer.
                // We now have 2 mutable references/pointers to this task. One inside
                // 
                RUNNING = task.borrow_mut();
            }
            // No task is found inside the queue 
            None => {}
        }
    }
    RUNNING
}

// Iterator implmentation for the `QueueIterator` type
impl<'a> Iterator for QueueIterator<'a> {
    type Item = &'a TaskTCB;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            node
        })
    }
}

unsafe impl Sync for Queue {}
