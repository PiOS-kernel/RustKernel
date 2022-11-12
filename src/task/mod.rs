use core::ptr;

//file to be reviewed, probably need to split it into modules, probably need to address some details

//defined type 
type TcbBlock = Option<Box<TaskTCB>>;           //used as a reference to a Task_TCB 
const STACK_SIZE: usize = 1024;                         //size of the stack for every task
type StPointer = Option<Box<usize>>;            //stack pointer

//global variables
pub const MAX_PRIORITY: u8 = 10;                    //max priority and size of the priority queues array
static mut running: TcbBlock;                       //variable for the running task.. best way to store it?

//definition of the Task Control Block 

struct TaskTCB {
    next: TcbBlock,                     //reference to the next Task_TCB
    priority: u8,                       //priority of the task
    stp: usize,                         //stack pointer
    stack: [u8; STACK_SIZE],            //stack associated to the task
}

impl TaskTCB {

    //constructor for a TaskTCB that return an instance of a TaskTCB 
    //with the associating the parameters to the corresponding fields
    pub fn new (n: TcbBlock, p: u8) -> Self{    
        Self { next: n, priority: p, stp: 0, stack: [u8; STACK_SIZE] }
    }
}

//struct of a queue of TaskTCB
struct Queue {
    head: TcbBlock,
    tail: *mut TaskTCB,
}

impl Queue {
    
    //initialize the queue with both head and tail None
    pub fn new () -> Self{
        Self {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    //return true if the queue is empty
    pub fn empty (&mut self) -> bool {
        self.head.is_none()
    }

    //enqueue a TaskTCB at the end of the queue
    pub fn enqueue (&mut self, block: TaskTCB) {
        let mut new_tail = Box::new(block);       //create a new Box<TaskTCB> pointing to the new element to add
        let tail_ptr: *mut _ = &mut *new_tail;    //create raw pointer to the new element just created

        if self.empty(){
            println!("{} is enqueued",new_tail.priority);       //if the queue is empty add the element in the head
            self.head = Some(new_tail);
        } else {
            println!("{} is enqueued",new_tail.priority);
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
            hprintln!("{} is dequeued", old_head.priority);
            Some(*old_head)              //return the popped element
        } else {
            hprintln!("the queue is empty");
            None                        //return None if the queue was already empty
        }
    }

    // scheduling function... rather simple for now considering only one queue and never ending tasks
    pub fn schedule(q : &Queue) {    

        if !q.empty() {
            let task = q.dequeue();             //take the first tasks in the queue and place it in last 
            //context_switch()                  //still need to implement the context switch part 
            q.enqueue(task.unwrap());
        } 
    }

}


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