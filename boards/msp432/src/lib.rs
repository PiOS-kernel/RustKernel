#![no_std]

use kernel::allocator::Heap;
use cortex_m4::mutex::Mutex;
use panic_halt as _;
use core::marker::PhantomData;

// The start address and size for the kernel's heap
pub const HEAP_START: usize = 0x2008000; // The middle of RAM's address space
pub const HEAP_SIZE: usize = 0x8000; // 32KB

static MUTEX: Mutex<Heap> = Mutex::<Heap>{data: PhantomData};
// The kernel's heap
#[global_allocator]
pub static HEAP: Heap = Heap::new(&MUTEX);