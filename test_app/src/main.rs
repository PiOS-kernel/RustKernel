#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod allocator_tests;
pub mod syscalls_tests;
pub mod task_tests;
pub mod utility_tests;

extern crate alloc;
use core::panic::PanicInfo;
use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::{hprint, hprintln};
use kernel::{kernel_init};

// 32KB in the .data section are dedicated to the heap
static mut HEAP_MEM: [u8; 0x8000] = [0; 0x8000];


#[entry]
fn _start() -> ! {
    // The kernel is initialized
    let heap_start = unsafe{ &HEAP_MEM[0] as *const u8 as usize };
    kernel_init(heap_start, 0x8000, 120000);    
    
    #[cfg(test)]
    test_main();

    loop {}
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    hprintln!("[failed]\n");
    hprintln!("Error: {}\n", info);
    loop {}
}

#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    // hprintln!("Unhandled exception: IRQn = {}", irqn);
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    hprintln!("\nHardFault, baby!");
    loop {}
}

pub fn test_runner(tests: &[&dyn Testable]) {
    hprintln!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    hprintln!("\nAll tests succeded.");
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        hprint!("{}...\t", core::any::type_name::<T>());
        self();
        hprintln!("[ok]");
    }
}
