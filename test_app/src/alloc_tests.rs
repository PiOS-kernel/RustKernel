use kernel::allocator::{Heap};
use cortex_m_semihosting::hprintln;

/* Utility function to display the free segments present in the heap */
fn print_heap(heap: Heap) {
    hprintln!();
    for seg in heap.iter() {
        hprintln!("start: {}, end: {}", seg.start_address(), seg.end_address());
    }
}

#[test_case]
fn heap_init_test() {
    let heap_mem: [u8; 1024] = [0; 1024];
    let mut heap = Heap::new();
    heap.init(&heap_mem[0] as *const u8 as usize, 1024);
    assert_eq!(heap.available_space(), 1024);
    assert_eq!(heap.count_segments(), 1);
}

#[test_case] 
fn count_segments_test() {
    let heap_mem: [u8; 1024] = [0; 1024];
    let mut heap = Heap::new();
    
    for i in 0..50 {
        let address = &heap_mem[20 * i] as *const u8 as usize;
        heap.add_free_segment(address, 20);
        assert_eq!(heap.count_segments(), i + 1);
    }
}

#[test_case]
fn available_space_test() {
    let heap_mem: [u8; 1024] = [0; 1024];
    let mut heap = Heap::new();
    heap.init(&heap_mem[0] as *const u8 as usize, 1024);
    
    for i in 0..50 {
        heap.allocate_segment(20);
        assert_eq!(heap.available_space(), 1024 - (i + 1) * 20);
    }
}

#[test_case]
fn heap_compaction_test() {
    let heap_mem: [u8; 1024] = [0; 1024];
    let mut heap = Heap::new();

    let mut address = &heap_mem[0] as *const u8 as usize;
    heap.add_free_segment(address, 128);
    assert_eq!(heap.count_segments(), 1);

    // TODO
}