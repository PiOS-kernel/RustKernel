
use kernel::{syscalls::create_task, WAITING_QUEUE};

fn foo(pippo : *mut u8) -> ! {
    loop {
        
    }
}

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
    assert_eq!(unsafe{ r15_ptr }, foo as *const usize);
}