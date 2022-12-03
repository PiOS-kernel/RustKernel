#![no_std]

use core::arch::asm;

use panic_halt as _;
use kernel::allocator::{Heap};
use kernel::syscalls::task_switch;
use cortex_m_rt::exception;


/*
    The SysTick handler is platform-agnostic
    Its initialization is performed in kernel_init() during boot routine
*/

static mut systick_counter: u8 = 0;
const TASK_TIME_UNIT: u8 = 10;

#[exception]
fn SysTick(){
    unsafe{ 
        systick_counter += 1;
        if systick_counter ==  TASK_TIME_UNIT{
            task_switch();
            unsafe{
                asm!("POP {{PC}}");
            }
            systick_counter = 0;
        }
    }
}