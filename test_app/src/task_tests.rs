use cortex_m_semihosting::hprintln;
use kernel::{HEAP};
use kernel::task::{Queue, TaskTCB, STACK_SIZE};
use alloc::boxed::Box;

#[test_case]
fn test_queue_empty() {
    let mut queue = Queue::new();
    assert_eq!(queue.empty(), true);
    assert_eq!(queue.count_tasks(), 0);
}

#[test_case]
fn test_enqueue() {
    let mut queue = Queue::new();

    for i in 0..5 {
        let new_task = Box::new(TaskTCB::new(None, i));
        queue.enqueue(new_task);
        assert_eq!(queue.count_tasks(), (i + 1) as usize);
    }
}

#[test_case]
fn test_dequeue() {
    let mut queue = Queue::new();

    for i in 0..5 {
        let new_task = Box::new(TaskTCB::new(None, i));
        queue.enqueue(new_task);
        assert_eq!(queue.count_tasks(), (i + 1) as usize);
    }

    for i in 0..5 {
        let task = queue.dequeue().unwrap();
        assert_eq!(task.priority, i);
        assert_eq!(queue.count_tasks(), (10 - i - 1) as usize);
    }
}

#[test_case]
fn test_stack_push() {
    let mut task_tcb = TaskTCB::new(None, 0);
    let mut buff: [u8; 5] = [1, 2, 3, 4, 5];
    let src = (&mut buff[0]) as *mut u8;
    let stp_old = task_tcb.stp;

    task_tcb.stack_push(src,5);


    let base = task_tcb.stp;
    for i in 0..5 {
        unsafe { assert_eq!(*base.offset(i as isize) , *src.offset(i as isize))}
    }

    assert_eq!(stp_old, unsafe{ task_tcb.stp.add(5) });
}

#[test_case]
fn test_stack_start() {
    let mut task_tcb = TaskTCB::new(None, 0);
    assert_eq!(task_tcb.stack_start(), unsafe{ &mut task_tcb.stack[0] as *mut u8});
}

#[test_case]
fn test_stack_end() {
    let mut task_tcb = TaskTCB::new(None, 0);
    assert_eq!(task_tcb.stack_end(), unsafe{ (&mut task_tcb.stack[0] as *mut u8).add(STACK_SIZE)});
}