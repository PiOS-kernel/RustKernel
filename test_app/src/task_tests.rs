use cortex_m_semihosting::hprintln;
use kernel::{HEAP};
use kernel::task::{Queue, TaskTCB};
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

    for i in 0..10 {
        let new_task = Box::new(TaskTCB::new(None, i));
        queue.enqueue(new_task);
        assert_eq!(queue.count_tasks(), (i + 1) as usize);
    }
}

#[test_case]
fn test_dequeue() {
    let mut queue = Queue::new();

    for i in 0..10 {
        let new_task = Box::new(TaskTCB::new(None, i));
        queue.enqueue(new_task);
        assert_eq!(queue.count_tasks(), (i + 1) as usize);
    }

    for i in 0..10 {
        let task = queue.dequeue().unwrap();
        assert_eq!(task.priority, i);
        assert_eq!(queue.count_tasks(), (10 - i - 1) as usize);
    }
}

#[test_case]
fn test_stack_push() {
    let mut task_tcb = TaskTCB::new(None, 0);
    let base = (&mut task_tcb.stack) as *mut u8;
    let mut buff: [u8; 5] = [1, 2, 3, 4, 5];
    let src = (&mut buff[0]) as *mut u8;
    let stp_old = task_tcb.stp;

    task_tcb.stack_push(src,5);

    for i in 0..5 {
        unsafe { assert_eq!(*base.offset(i as isize) , *src.offset(i as isize))}
    }

    assert_eq!(stp_old, task_tcb.stp - 5);
}

#[test_case]
fn test_get_stp() {
    let mut task_tcb = TaskTCB::new(None, 0);
    let mut buff: [u8; 5] = [1, 2, 3, 4, 5];
    let src = (&mut buff[0]) as *mut u8;

    task_tcb.stack_push(src, 5);
    let stp = task_tcb.get_stp();

    unsafe { assert_eq!(*stp.sub(1), 5) };
}