use core::arch::asm;

use cortex_m_semihosting::hprintln;
use kernel::syscalls::{create_task, task_switch, kcreate_task};
use kernel::{WAITING_QUEUE};
use kernel::task::{TaskTCB, RUNNING};
use alloc::boxed::Box;

const ARGS_PTR: *mut u8 = 123 as *mut u8;


#[test_case]
fn test_create_task() {
    create_task(mock_task, ARGS_PTR, 0);
    assert_eq!(WAITING_QUEUE.count_tasks(), 1);

    let mut created_task = WAITING_QUEUE.dequeue().unwrap();

    let stack_top = created_task.stp as *mut usize;


    // First we should find the task's arguments
    assert_eq!(unsafe{ *stack_top }, ARGS_PTR as usize);

    // The following 12 words from the top of the stack (registers r1-r12) should be 0-filled
    for i in 1..13 {
        let reg_ptr = unsafe{ stack_top.add(i) };
        assert_eq!(unsafe{ *reg_ptr }, 0);
    }

    // Then we should find the link register
    assert_eq!(unsafe{ *stack_top.add(13) }, mock_task as usize);
}

fn accumulate(base: usize) -> usize {
    let mut array: [usize; 100] = [0; 100];
    let mut acc = base;
    for i in (base + 1)..(base+100) {
        array[i - base] = i;
        assert_eq!(array[i], i);

        acc += i;
    }
    acc
}

fn mock_task(args: *mut u8) {
    hprintln!("it's me, Luigi! {:#x}", ARGS_PTR as usize);

    assert_eq!(args, ARGS_PTR);

    let var = accumulate(0);
    assert_eq!(var, 4950);
}

/*
This test does not run to completion because it hands control to
'mock_task', therefore it should not be run together with other tests
*/

/*
#[test_case]
fn test_task_switch() {
    // the waiting queue is emptied
    while !WAITING_QUEUE.empty() {
        WAITING_QUEUE.dequeue();
    }

    // a new task is created
    kcreate_task(mock_task, ARGS_PTR, 0);

    // context switch
    unsafe {
        RUNNING = Some(Box::new(TaskTCB::new(None, 0)));
        task_switch();
        asm!("POP {{pc}}");
    };
}
*/
