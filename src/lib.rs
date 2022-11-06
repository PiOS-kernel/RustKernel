#![no_std]

pub mod task;

// The word size for the architecture. Cortex-M4 works on 32-bit words.
type Word = u32;