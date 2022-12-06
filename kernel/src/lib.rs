#![no_std]
#![feature(const_mut_refs)]
#![feature(alloc_error_handler)]
#![feature(asm_const)]
#![feature(naked_functions)]

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
#[macro_use(exception)]


// The kernel's heap and the task queue
// The `mut` keyword prevents the linker from placing those variables in FLASH
// memory, as it would assume them to be read-only. If that was the case,
// mutating those objects would silently fail.
#[global_allocator]
static mut heap: LockedHeap = LockedHeap::new();
pub static HEAP: &LockedHeap = unsafe{&heap};
static mut waiting_queue: LockedQueue = LockedQueue::new();
pub static WAITING_QUEUE: &LockedQueue = unsafe{&waiting_queue};
static boo: u8 = 10;
static varr: u8 = 11;
static mut HEAP_MEMORY: [u8; 0x8000] = [0; 0x8000];
pub const HEAP_SIZE: usize = 0x8000;

use cortex_m::peripheral::syst::SystClkSource;

/* ------- */

#[no_mangle]
pub extern "C" fn kernel_init(reload_value: u32){
    unsafe{
        // let heap_start = &HEAP_MEMORY[0] as *const u8 as usize;
        // HEAP.init(heap_start, HEAP_SIZE);
        let var2 : u8 = varr;
        HEAP_MEMORY[0] = 19;
    }
}

#[no_mangle]
pub extern "C" fn get_addr() -> usize {
    unsafe{
        &HEAP_MEMORY[0] as *const u8 as usize
    }
}
/*---------- */

// The kernel initialization routine, for the time being it just 
// initializes the heap and the systick peripheral
// #[no_mangle]
// pub extern "C" fn kernel_init(heap_start : usize, heap_size : usize,  reload_value : u32) {
//     unsafe{
//         HEAP.init(heap_start, heap_size);
//     }

//     //systick init
//     // let p = cortex_m::Peripherals::take().unwrap();
//     // let mut syst = p.SYST;
//     // syst.set_clock_source(SystClkSource::Core);
//     // // this is configured for the LM3S6965 which has a default CPU clock of 12 MHz
//     // syst.set_reload(reload_value);

//     // // questi tre da fare una volta che il kernel è inizializzato
//     // syst.clear_current();
//     // syst.enable_counter();
//     // syst.enable_interrupt();
// }

#[exception]
fn SVCall(){
    unsafe{
        asm!(
            "ldr r4, [r7, #40]",
            "ldrb r4, [r4, #-2]",
            "cmp r4, #01",
            "itt eq",
            "ldreq r5, =kcreate_task",
            "beq 2f",
            "ldr r5, =unknownService",
            "2:",
            "str lr, [sp, #-4]!",
            "blx r5",
            "ldr pc, [sp], #4",
        );
    }
}

/* Add this code block when implementing a new service

    "cmp r0, #numeric_code",
    "itt eq",
    "ldreq r1, =service_name",
    "beq 2f",

*/


/* Possibili cause se SVCall non dovesse funzionare
- potrebbe essere la "risalita" dello stack - invece di essere 24 (6 registri da 4 byte), è 40 - probabilmente viene pushato altro
- test di quale stack pointer è attualmente in uso - ora non viene fatto, viene usato sp salvato in r7 (ha senso? è giusto?)
        non dovrebbe servire perchè in r7 è salvato il current stack pointer
        "tst lr, #4",
        "ite eq",
        "mrseq r0, msp",
        "mrsne r0, psp",

*/