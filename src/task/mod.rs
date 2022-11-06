use super::Word;

const STACK_SIZE: usize = 4096;

struct Task {
    // Stack pointer for the given task
    sp: Word,

    // Priority of the given task
    priority: usize,

    // The task's stack
    stack: [Word; STACK_SIZE],
}