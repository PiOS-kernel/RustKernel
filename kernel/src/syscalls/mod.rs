use crate::WAITING_QUEUE;
use crate::task::{TaskTCB, RUNNING};
use core::mem::size_of;
use core::arch::asm;
use alloc::boxed::Box;
use cortex_m_semihosting::{hprint, hprintln};
use cortex_m::interrupt::disable;

/* 
This enum lists all the services that can be requested by an application to 
the kernel.
Each service has a numeric identifier.
*/
pub enum SysCallID {
    CREATE_TASK_ID = 1,
}

/* 
This is the system call provided to the user application, in order to
create a new task.

It accepts a function pointer, a pointer to its arguments, and a priority.

The function simply invokes the kernel to request the given service.
*/
#[no_mangle]
#[naked]
pub fn create_task(code: fn(*mut u8), args: *mut u8, priority: usize) {
    unsafe {
        asm!(
            "svc {syscall_id}",
            "mov pc, lr",
            syscall_id = const SysCallID::CREATE_TASK_ID as u8,
            options(noreturn)
        );
    }
}

#[no_mangle]
pub(crate) fn unknownService(){
    loop {
        for i in 0x0..0xFFFFF {
            // busy waiting
        }
    }
 }


/*
- kcreate_task(), brief description:
    This is the function used by the kernel to create a new task
    The functions pushes onto the task's empty stack the initial values
    for its register. Then the task is added to the tasks queue.

- Registers layout for the cortex-M4 processor:

    r0  function argument 1 / general purpose
    r1  function argument 2 / general purpose
    r2  function argument 3 / general purpose
    r3  function argument 4 / general purpose

    r4  --
    r5   |
    r6   |
    r7   |
    r8   |  General purpose
    r9   |
    r10  |
    r11  |
    r12 --

    r13 stack pointer
    r14 link register
    r15 program counter

- Task initialization:

    When a task is prehempted by the SysTick interrupt handler, its 
    program counter is saved in the link register. Then the SysTick 
    handler calls the routine that performs the context switch, which
    pushes r0 through r12, and r14 (link register) onto the stack.
        
    Therefore a new task's stack needs to be initialized by pushing 
    the necessary values for registers r0-r12 and for the link register, 
    which should hold the memory address of the first instruction to be
    executed by the task. 
*/
#[no_mangle]
pub fn kcreate_task(code: fn(*mut u8), args: *mut u8, priority: usize) {
    // The task's TCB is created
    let mut tcb = TaskTCB::new(None, priority); 


    // The link register is pushed onto the stack, and initialized to be 
    // the memory address of the first instruction executed by the task
    tcb.stack_push(&code as *const fn(*mut u8) as *mut u8, size_of::<*mut u8>());


    // Registers r1 through r12 are pushed onto the stack and 
    // 0-initialized.
    // 12 * 4 bytes are copied to the stack, where 4 bytes is the size of 
    // one register.

    let zeros: [usize; 12] = [0; 12];
    // The memory address of the first item in the array is given as source
    tcb.stack_push(&zeros[0] as *const usize as *const u8, size_of::<usize>() * 12);


    // The pointer to the arguments is saved in register r0.
    // The ARM ABI specifies that the first 4 32-bit function arguments
    // should be put in registers r0-r3.

    tcb.stack_push(&args as *const *mut u8 as *const u8, size_of::<*mut u8>());

    let mut heap_allocated_tcb = Box::new(tcb);
    heap_allocated_tcb.stp = unsafe{ heap_allocated_tcb.stack_end().sub(14 * 4) };

    // The task is inserted into the tasks queue
    WAITING_QUEUE.enqueue(heap_allocated_tcb);
}

//this function does the context switch for a task
//stores the current values in the registers to the current task's stack
//calls the schedule function
//and loads the new task's stack in the registers
#[no_mangle]
#[naked]
#[cfg(target_arch = "arm")]
pub unsafe fn task_switch() {
    asm!(
        // Interrupts are disabled
        "CPSID i",
        // The 'Rust' part of this function is called, to get the pointer to
        // the running task
        "STMDB r13!, {{r14}}",
        "BL task_switch_prologue",
        "LDMIA r13!, {{r14}}",

        /*
        SAVE:
        At this point, r0 holds the pointer to the running task. Because
        the first 32 bits of the TaskTCB struct are dedicated to the stack
        pointer, the value of r13 will be saved at that memory location before
        context switching
        */
        // If there is currently no running task, skip the SAVE part and
        // branch to the scheduler
        "CMP r0, #0",
        "BEQ 2f",
        // The task's registers are saved onto the stack
        "STMDB r13!, {{r4-r12, r14}}",   
        // the stack pointer is loaded in r13 (sp register)               
        "STR r13, [r0]",

        /*
        SCHEDULING:
        the scheduling algorithm determines wich task should be executed
        */
        "2:",
        "STMDB r13!, {{r14}}",
        "BL schedule",
        "LDMIA r13!, {{r14}}",

        /*
        RESUME:
        according to the ARM ABI convention the return value of 'schedule()',
        which is the pointer to the new running task, is saved in register r0
        */
        // the first struct field is the SP
        "LDR r13, [r0, #0]",
        // the task's registers are popped from the stack
        "LDMIA r13!, {{r0-r12}}",

        // Interrupts are enabled again
        "CPSIE i",
        // The register that tracks the current privilege level of the CPU
        // is modified to return to user mode
        "STMDB r13!, {{r0}}",
        "MOV r0, #1",
        "MSR basepri, r0",
        "ISB",
        "LDMIA r13!, {{r0}}",
        // At the top of the stack there is the return address to the task code
        "MOV pc, lr",     
        options(noreturn)
    );  
}

/*
This function serves as prologue to task_switch(), and it
returns pointer to the currently running task. It is needed
to correctly handle return from task_switch to the caller.
*/

#[no_mangle]
#[cfg(target_arch = "arm")]
pub unsafe extern "C" fn task_switch_prologue() {
    use core::ptr;

    let mut running_ptr = match &mut RUNNING {
        Some(tcb) => {
            &mut **tcb
        }
        None => {
            0 as *mut TaskTCB
        }
    };

    // The pointer is saved into r0, the return register
    asm!(
        "",
        inout("r0") running_ptr,
    );
}