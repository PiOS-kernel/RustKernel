use core::mem;
use crate::mutex::MutexGuard;
use core::cmp::max;
use super::mutex::Mutex;

const HEAP_SEG_HEADER_SIZE: usize = mem::size_of::<HeapSegment>();

type SegmentLink = Option<&'static mut HeapSegment>;

/*
HeapSegments are the 'header' of each memory block that is NOT allocated on the
Heap. This representation allows us to store the heap as a linked list of 
memory segments.
*/

pub struct HeapSegment {
    size: usize,
    next: SegmentLink,
}

/*
The Heap simply contains a reference to the first available block of memory
*/

pub struct Heap {
    head: SegmentLink,
}

/*
This type wraps an Heap into a mutex, providing mutual access to it. It is
needed to implement the trait `GlobalAllocator`, more on GlobalAllocator at
the end of the file.
*/

pub struct LockedHeap {
    heap: Mutex<Heap>,
}

/*
HeapIterator is used to iterate through the heap. More on that towards the
end of the file.
*/

pub struct HeapIterator<'a> {
    next: Option<&'a HeapSegment>
}

impl LockedHeap {
    pub const fn new() -> Self {
        Self { heap: Mutex::new(Heap::new()) }
    }

    pub fn lock(&self) -> MutexGuard<'_, Heap> {
        self.heap.lock()
    }

    pub fn init(&self, start_address: usize, size: usize) {
        self.lock().add_free_segment(start_address, size);
    } 
}

impl Heap {

    pub const fn new() -> Self {
        Self { head: None }
    }

    /* Initializes the heap as a single empty memory block */

    pub fn init(&mut self, start_address: usize, size: usize) {
        self.add_free_segment(start_address, size);
    }

    /* Creates the iterator */

    pub fn iter(&self) -> HeapIterator {
        HeapIterator { next: self.head.as_deref() }
    }

    /* Allocates to the caller a memory segment */

    pub fn allocate_segment(self: &mut Self, size: usize) -> Option<*mut u8> {
        // There is no available memory left
        if self.head.is_none() {
            return None;
        }
        
        // The heap never allocates segments smaller than HEAP_SEG_HEADER_SIZE
        let actual_size = max(size,HEAP_SEG_HEADER_SIZE);

        // Check the head first
        if self.head.as_ref().unwrap().size >= actual_size {
            let old_head = self.head.take().unwrap();

            // The segment is split into two new ones, and the firt one is 
            // allocated
            Self::trim_segment(old_head, actual_size);
            self.head = old_head.next.take();
            return Some(old_head.start_address() as *mut u8);
        }

        let mut cursor = self.head.as_mut().unwrap();
        let mut advance = true;

        // Iterate through the list until a large enough block/segment is found
        while advance {
            advance = match cursor.next.as_ref() {
                None => {
                    // The end of the list is reached, there is no large
                    // enough segment available
                    return None;
                }
                Some(next) => {
                    next.size < actual_size
                }
            };
            if advance {
                cursor = cursor.next.as_mut().unwrap();
            }
        }
        
        // The segment is split into two new ones, and the firt one is 
        // allocated
        let next = cursor.next.take().unwrap();
        Self::trim_segment(next, actual_size);
        cursor.next = next.next.take();
        
        Some(next.start_address() as *mut u8)
    }

    /* 
    When a segment is freed, it is put back into the list of
    free segments
    */

    pub fn free_segment(self: &mut Self, start_address: usize, size: usize) {
        self.add_free_segment(start_address, size);

        // Ajacent segments are merged
        self.compaction();
    }

    /* 
    The functions inserts a segment into the free list, in the correct
    position
    */

    pub fn add_free_segment(self: &mut Self, address: usize, size: usize) {     
        // The heap should never allocate segments of size less than
        // HEAP_SEG_HEADER_SIZE
        assert!(size > HEAP_SEG_HEADER_SIZE);
        
        let mut new_seg = unsafe{Self::init_segment(HeapSegment::new(size), address)};
        if self.head.is_none() || self.head.as_ref().unwrap().start_address() > address {
            new_seg.next = self.head.take();
            self.head = Some(new_seg);
            return;
        }

        let mut cursor = self.head.as_mut().unwrap();
        let mut advance = true;
        while advance {
            // Iterate through the list until a segment starting at a greater address 
            // than the new one is found

            advance = match cursor.next.as_ref() {
                None => {
                    false
                }
                Some(next) => {
                    next.start_address() < address
                }
            };
            if advance {
                cursor = cursor.next.as_mut().unwrap();
            } else {
                // The segment is inserted into the list
                new_seg.next = cursor.next.take();
            }
        }
        cursor.next = Some(new_seg);
    }

    /*
    The function looks for adjecent segments and merges them into a single one
    */

    pub fn compaction(self: &mut Self) {
        if self.head.is_none() {
            return;
        }

        let mut cursor = self.head.as_mut().unwrap();
        loop {
            let node_start = cursor.start_address();
            let compacted = match cursor.next.as_mut() {
                None => {
                    // The end of the list was reached, there are no more
                    // segments to merge
                    return;
                }
                Some(next) => {
                    // If the following segment starts the byte after the 
                    // end of the current segment, the two are merged

                    if next.start_address()
                        == node_start + cursor.size
                    {
                        cursor.size = cursor.size + HEAP_SEG_HEADER_SIZE + next.size;
                        cursor.next = next.next.take();
                        true
                    } else {
                        false
                    }
                }
            };

            // If two segmetns were merged, the cursor does not need to be
            // advanced, as it might be possible to merge the following 
            // segment
            if !compacted {
                cursor = cursor.next.as_mut().unwrap();
            }
        }
    }

    /* Utility function to compute the total available space in the heap */

    pub fn available_space(&self) -> usize {
        let mut total = 0;
        for seg in self.iter() {
            total += seg.size;
        }
        total
    }

    /* Utility function that returns the number of free segments in the heap */

    pub fn count_segments(&self) -> usize {
        let mut total = 0;
        for sef in self.iter() {
            total += 1;
        }
        total
    }

    /*
    This function copies an `HeapSegment` struct at the desired address, while
    returning a mutable reference to it.
    */

    pub unsafe fn init_segment(seg: HeapSegment, address: usize) -> &'static mut HeapSegment {
        let address_ptr = address as *mut HeapSegment;
        address_ptr.write(seg);
        &mut *address_ptr
    }
 
    /*
    The function trims down a segment splitting it into two new ones of sizes
    <target_size> and <size - target_size>
    */

    pub fn trim_segment(seg: &mut HeapSegment, target_size: usize) {
        let new_seg_addr = seg.start_address() + target_size;
        let new_seg_size = seg.size - target_size;

        // The segment gets trimmed only if both the new segments would
        // be larger than HEAP_SEG_HEADER_SIZE
        if new_seg_size > HEAP_SEG_HEADER_SIZE {
            seg.size = target_size;
            let mut new_seg = unsafe{Self::init_segment(HeapSegment::new(new_seg_size), new_seg_addr)};
            new_seg.next = seg.next.take();
            seg.next = Some(new_seg);
        }
    }
}

/* 
HeapSegments 
*/

impl HeapSegment {
    pub const fn new(size: usize) -> Self {
        Self { size, next: None }
    }
    pub fn start_address(self: &Self) -> usize {
        self as *const Self as usize
    }
    pub fn end_address(self: &Self) -> usize {
        self as *const Self as usize + self.size
    }
}

/* 
HeapIterator implements the Iterator trait, which allows us to iterate
through heap segments with the `for el in HEAP` construct
*/

impl<'a> Iterator for HeapIterator<'a> {
    type Item = &'a HeapSegment;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            node
        })
    }
}

/* 
LockedHeap implments the GlobalAlloc interface. Because that allows Rust 
to know how to allocate memory dynamically, we can use standard library types
like `Box`, `Vec`, ... and so on.
*/

use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr;

unsafe impl GlobalAlloc for LockedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut heap = self.lock();

        match heap.allocate_segment(layout.size()) {
            None => ptr::null_mut(),
            Some(ptr) => ptr
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut heap = self.lock();
        heap.add_free_segment(_ptr as usize, _layout.size());
    }
}

/* The allocation error handler, needed by the `alloc` crate */

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout);
    loop{}
}