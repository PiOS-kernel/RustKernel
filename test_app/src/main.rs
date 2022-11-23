#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod alloc_tests;
pub mod task_tests;
pub mod syscalls_tests;
pub mod utility_tests;

use cortex_m_rt::entry;
#[cfg(not(test))]
use panic_halt as _;

#[entry]
fn _start() -> ! {
    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(test)]
pub mod test {
    use cortex_m_semihosting::{hprint, hprintln};
    use core::panic::PanicInfo;
    use cortex_m_rt::{exception};

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

    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        hprintln!("[failed]\n");
        hprintln!("Error: {}\n", info);
        loop {}
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
}
