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
use allocator::LockedHeap;
use task::LockedQueue;

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