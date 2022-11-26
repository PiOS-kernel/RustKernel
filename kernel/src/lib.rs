#![no_std]
#![feature(const_mut_refs)]
#![feature(alloc_error_handler)]
#![feature(asm_const)]

extern crate alloc;
pub mod allocator;
pub mod mutex;
pub mod task;
pub mod syscalls;
pub mod utility;
use core::arch::asm;

use allocator::LockedHeap;
use task::LockedQueue;

use cortex_m_rt::exception;
use cortex_m_semihosting::{hprint, hprintln};

use syscalls::kcreate_task;

#[macro_use(exception)]

// The memory for the heap is allocated at compile time
static mut HEAP_MEMORY: [u8; 0x8000] = [0; 0x8000];
pub const HEAP_SIZE: usize = 0x8000; // 32KB

// The kernel's heap and the task queue
// The `mut` keyword prevents the linker from placing those variables in FLASH
// memory, as it would assume them to be read-only. If that was the case,
// mutating those objects would silently fail.
#[global_allocator]
static mut heap: LockedHeap = LockedHeap::new();
pub static HEAP: &LockedHeap = unsafe{&heap};
static mut waiting_queue: LockedQueue = LockedQueue::new();
pub static WAITING_QUEUE: &LockedQueue = unsafe{&waiting_queue};

// The kernel initialization routine, for the time being it just 
// initializes the heap
pub unsafe fn kernel_init() {
    let heap_start = &HEAP_MEMORY[0] as *const u8 as usize;
    HEAP.init(heap_start, HEAP_SIZE);
}

fn foo(){}

#[exception]
fn SVCall(){
    // let number : i32;
    // hprintln!("inside SVC");
    unsafe{
        //foo();
        asm!(
            "mov r0, r7",
            "ldr r0, [r0, #40]",
            "ldrb r0, [r0, #-2]",
            "cmp r0, #01",
            "itt eq",
            "ldreq r1, =kcreate_task",
            "beq 2f",
            "cmp r0, #02",
            "itt eq",
            "ldreq r1, =test2",
            "beq 2f",
            "ldr r1, =unknownService",
            "2:",
            "str lr, [sp, #-4]!",
            "blx r1",
            "ldr pc, [sp], #4",
        );
    }
}

/* Possibili cause se SVCall non dovesse funzionare
- potrebbe essere la "risalita" dello stack - invece di essere 24 (6 registri da 4 byte), è 40 - probabilmente viene pushato altro
- test di quale stack pointer è attualmente in uso - ora non viene fatto, viene usato sp salvato in r7 (ha senso? è giusto?)
        non dovrebbe servire perchè in r7 è salvato il current stack pointer
        "tst lr, #4",
        "ite eq",
        "mrseq r0, msp",
        "mrsne r0, psp",

*/