use core::mem;
use super::Word;

const HEAP_SEG_HEADER_SIZE: usize = mem::size_of::<HeapSegment>();

type SegmentLink = Option<&'static mut HeapSegment>;

pub struct HeapSegment {
    size: usize,
    next: SegmentLink,
}

pub struct Heap {
    head: SegmentLink,
}

impl HeapSegment {
    pub const fn new(size: usize) -> Self {
        Self { size, next: None }
    }
    pub fn start_address(self: &Self) -> Word {
        self as *const Self as Word
    }
    pub fn end_address(self: &Self) -> Word {
        self as *const Self as Word + (self.size + HEAP_SEG_HEADER_SIZE) as Word
    }
}

impl Heap {
    pub const fn new() -> Self {
        Self { head: None }
    }
    pub fn init(&mut self, start_address: Word, size: usize) -> Self {
        let mut heap = Self {head: None};
        heap.add_free_segment(start_address, size);
        heap
    }
    pub fn allocate_segment(self: &mut Self, size: usize) -> Option<&'static HeapSegment> {
        if self.head.is_none() {
            return None;
        }
        assert!(size > HEAP_SEG_HEADER_SIZE);

        let actual_size = size - HEAP_SEG_HEADER_SIZE;
        if self.head.as_ref().unwrap().size >= size {
            let mut head = self.head.take().unwrap();
            Self::trim_segment(head, actual_size);
            self.head = head.next.take();
            return Some(head);
        }

        let mut cursor = self.head.as_mut().unwrap();
        let mut advance = true;
        while advance {
            advance = match cursor.next.as_ref() {
                None => {
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
        
        let mut next = cursor.next.take().unwrap();
        Self::trim_segment(next, actual_size);
        cursor.next = next.next.take();
        
        self.compaction();
        Some(next)
    }
    pub fn free_segment(self: &mut Self, seg: &HeapSegment) {
        self.add_free_segment(seg.start_address(), seg.size + HEAP_SEG_HEADER_SIZE);
        self.compaction();
    }
    fn add_free_segment(self: &mut Self, address: Word, size: usize) {
        assert!(size > 0);
        
        let mut new_seg = unsafe{Self::init_segment(HeapSegment::new(size - HEAP_SEG_HEADER_SIZE), address)};
        if self.head.is_none() || self.head.as_ref().unwrap().start_address() > address {
            new_seg.next = self.head.take();
            self.head = Some(new_seg);
            return;
        }

        let mut cursor = self.head.as_mut().unwrap();
        let mut advance = true;
        while advance {
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
                new_seg.next = cursor.next.take();
            }
        }
        cursor.next = Some(new_seg);
    }
    fn compaction(self: &mut Self) {
        if self.head.is_none() {
            return;
        }

        let mut cursor = self.head.as_mut().unwrap();
        loop {
            let node_start = cursor.start_address();
            let compacted = match cursor.next.as_mut() {
                None => {
                    return;
                }
                Some(next) => {
                    if next.start_address()
                        == node_start + (HEAP_SEG_HEADER_SIZE + cursor.size) as Word
                    {
                        cursor.size = cursor.size + HEAP_SEG_HEADER_SIZE + next.size;
                        cursor.next = next.next.take();
                        true
                    } else {
                        false
                    }
                }
            };
            if !compacted {
                cursor = cursor.next.as_mut().unwrap();
            }
        }
    }
    unsafe fn init_segment(seg: HeapSegment, address: Word) -> &'static mut HeapSegment {
        let address_ptr = address as *mut HeapSegment;
        address_ptr.write(seg);
        &mut *address_ptr
    }
    fn trim_segment(seg: &mut HeapSegment, target_size: usize) {
        let new_seg_addr = seg.start_address() + (HEAP_SEG_HEADER_SIZE + target_size) as Word;
        let new_seg_size = seg.size - target_size;
        if new_seg_size > HEAP_SEG_HEADER_SIZE {
            seg.size = target_size;
            let mut new_seg = unsafe{Self::init_segment(HeapSegment::new(new_seg_size - HEAP_SEG_HEADER_SIZE), new_seg_addr)};
            new_seg.next = seg.next.take();
            seg.next = Some(new_seg);
        }
    }
}