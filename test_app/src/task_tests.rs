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