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
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::{hprint, hprintln};
use kernel::{kernel_init};

#[entry]
fn _start() -> ! {
    // The kernel is initialized
    unsafe{ kernel_init() };
    
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
    hprintln!("Unhandled exception: IRQn = {}", irqn);
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
