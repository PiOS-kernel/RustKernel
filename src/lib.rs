#![no_std]
#![feature(const_mut_refs)]

extern crate alloc;

pub mod allocator;
pub mod mutex;

// The word size for the architecture. Cortex-M4 works on 32-bit words.
type Word = u32;

// The start address and size for the kernel's heap
pub const HEAP_START: Word = 0x2008000; // The middle of RAM's address space
pub const HEAP_SIZE: usize = 0x8000; // 32KB