use kernel::allocator;

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 2);
}