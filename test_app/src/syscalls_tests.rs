
use core::borrow::BorrowMut;

use cortex_m_semihosting::hprintln;
use kernel::{syscalls::{create_task, task_switch}, WAITING_QUEUE, task::{TaskTCB, RUNNING}};
use alloc::boxed::Box;

fn foo(pippo : *mut u8) -> ! {
    loop {
        
    }
}
 /*
#[test_case]
fn test_create_task() {
    let args_ptr = 0;
    create_task(foo, unsafe{ args_ptr as *mut u8 }, 0);
    assert_eq!(WAITING_QUEUE.count_tasks(), 1);

    let mut created_task = WAITING_QUEUE.dequeue().unwrap();
    // r0 should contain the pointer to the tasks arguments
    let r0_ptr = unsafe{ &created_task.stack[0] as *const u8 as *const usize };
    assert_eq!(unsafe{ *r0_ptr }, args_ptr);
    
    // registers r1 - r14 should be 0-filled
    for i in 1..15 {
        let reg_ptr = unsafe{ r0_ptr.add(i) };
        assert_eq!(unsafe{ *reg_ptr }, 0);
    }

    // r15(pc) should contain the pointer to the task function
    let r15_ptr = unsafe{ r0_ptr.add(15) };
    assert_eq!(unsafe{ *r15_ptr }, foo as usize);
}
*/
#[test_case]
fn test_task_switch() {
    //create 2 tasks 
    let mut task2 = TaskTCB::new(None, 2);
    let mut task1 = TaskTCB::new(None, 0);

    //add values to the task's stack
    let mut buff: [u8; 5] = [1, 2, 3, 4, 5];
    let src = (&mut buff) as *mut u8;
    task2.stack_push(src, 5);

    //add task2 to the waiting queue 
    WAITING_QUEUE.enqueue(Box::new(task2));

    //set task1 as the RUNNING task and call task_switch()
    unsafe { 
        RUNNING = task1.borrow_mut();
        task_switch();
    };

    //RUNNING should point to task2
    unsafe { assert_eq!((*RUNNING).priority, 2) };

    //WAITING_QUEUE should have only one task
    assert_eq!(WAITING_QUEUE.count_tasks(), 1);

}