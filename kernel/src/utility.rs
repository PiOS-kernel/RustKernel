/* The great memcpy */
pub unsafe fn memcpy(src: *mut u8, dst: *mut u8, len: usize) {
    for i in 0..len {
        *dst.offset(i as isize) = *src.offset(i as isize);
    }
}