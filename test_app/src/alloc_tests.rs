use kernel::allocator::{Heap};
use cortex_m_semihosting::hprintln;

fn print_heap(heap: Heap) {
    hprintln!();
    for seg in heap.iter() {
        hprintln!("start: {}, end: {}", seg.start_address(), seg.end_address());
    }
}

#[test_case]
fn trivial_assertion() {
    let heap_mem: [u8; 1024] = [0; 1024];
    let mut heap = Heap::new();
    heap.init(unsafe{&heap_mem as *const [u8] as *const u8 as usize}, 1024);

    print_heap(heap);
    assert_eq!(1, 1);
}