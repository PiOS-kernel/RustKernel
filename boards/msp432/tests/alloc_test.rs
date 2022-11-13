#![no_std]
#![no_main]

use defmt_rtt as _;

// See https://crates.io/crates/defmt-test/0.3.0 for more documentation (e.g. about the 'state'
// feature)
#[defmt_test::tests]
mod tests {
    use core::panic::PanicInfo;
    use cortex_m_semihosting::hprintln;
    use panic_halt as _;

    #[test]
    fn it_works() {
        hprintln!("Ce l'abbiamo");
    }
}