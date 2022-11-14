#![no_std]
#![feature(const_mut_refs)]
#![feature(alloc_error_handler)]

extern crate alloc;
pub mod allocator;
pub mod mutex;
pub mod task;
use allocator::LockedHeap;
use task::Queue;
use panic_halt as _;

// The word size for the architecture. Cortex-M4 works on 32-bit words.
type Word = u32;

// The start address and size for the kernel's heap
pub const HEAP_START: Word = 0x2008000; // The middle of RAM's address space
pub const HEAP_SIZE: usize = 0x8000; // 32KB

// The kernel's heap
#[global_allocator]
pub static HEAP: LockedHeap = LockedHeap::new();

// The tasks queue
pub static TASKS_QUEUE: Queue = Queue::new();