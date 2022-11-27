use crate::WAITING_QUEUE;
use crate::task::{TaskTCB, RUNNING};
use core::mem::size_of;
use core::arch::asm;
use alloc::boxed::Box;
use cortex_m_semihosting::{hprint, hprintln};

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
pub extern "C" fn create_task(code: fn(*mut u8)->!, args: *mut u8, priority: u8) {
    unsafe {
        asm!("svc {syscall_id}", syscall_id = const SysCallID::CREATE_TASK_ID as u8);
    }
}

#[no_mangle]
pub(crate) fn unknownService(){
    loop {
        hprintln!("SVC called unknown service");
        for i in 0x0..0xFFFFF {
            // busy waiting
        }
    }
 }

/*
This is the function used by the kernel to create a new task

The functions pushes onto the task's empty stack the initial values
for its register. Then the task is added to the tasks queue.

The registers layout for the cortex-M4 processor is the following:

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

    TODO: address the need to initialize and store control registers
*/
#[no_mangle]
pub(crate) fn kcreate_task(code: fn(*mut u8), args: *mut u8, priority: u8) {
    // The task's TCB is created
    let mut tcb = TaskTCB::new(None, priority);

    // The pointer to the arguments is saved in register r0.
    // The ARM ABI specifies that the first 4 32-bit function arguments
    // should be put in registers r0-r3.
    tcb.stack_push(args as *mut u8, size_of::<*mut u8>());

    // The following 11 general purpose registers, the stack pointer and
    // the link register are 0-filled. The stack pointer will be
    // initialized the first time the task is executed.
    let mut zeros: [u8; 14] = [0; 14];
    tcb.stack_push(&mut zeros[0] as *mut u8, size_of::<u8>() * 14);

    // The program counter is saved with the pointer to the task's code
    tcb.stack_push(&code as *const fn(*mut u8) as *mut u8, size_of::<*mut u8>());

    // The task is inserted into the tasks queue
    WAITING_QUEUE.enqueue(Box::new(tcb));
}

//this function does the context switch for a task
//stores the current values in the registers to the current task's stack
//calls the schedule function
//and loads the new task's stack in the registers
#[no_mangle]
pub unsafe fn task_switch() {             
    asm!(
        //SAVE: 
        "STMFD r13!, {{r0-r12, r14}}", 
        //"LDR r0, RUNNING",             r0 is inizialized as an input registeer containing the running running  
        "LDR r1, [r0, #0]",              // r1<-runningPROC                  come è salvata la struct in memoria? 
        "STR r13, [r1, #1]",             // running->ksp = sp                Quanti byte di offset per sp?
        //FIND:
        "BL schedule",                   // call schedule()                  
        //RESUME:
        //arm convention save in r0 the return value of schedule which is the pointer to the new running task
        "LDR r1, [r0, #0]",              // r1<-new running PROC
        "LDR r13, [r1, #1]",             // restore running->ksp
        "LDMFD r13!, {{r0-r12, r14}}",   // restore register
        "MOV pc, lr",                    // return             
        in("r0") RUNNING,                //initialize r0 with the running pointer
    );  // rust-analyzer highlights an error because since the
    // target architecture is not configured, it is using the host's assembler
}