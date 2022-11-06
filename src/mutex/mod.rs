use core::arch::asm;
use core::ops::{Deref, DerefMut};

pub struct Mutex<T> {
    inner: T,
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Mutex<T> {
    pub fn new(inner: T) -> Self {
        Self {inner: inner}
    }

    pub fn lock(&self) -> MutexGuard<T> {
        unsafe {
            asm!("cpsid"); // Disable interrupts
            MutexGuard::new(self)
        } 
    }
}

impl<'a, T> MutexGuard<'a, T> {
    pub fn new(mutex: &'a Mutex<T>) -> Self {
        Self {mutex: mutex}
    }
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { & *(&self.mutex.inner as *const T) }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *(&self.mutex.inner as *const T as *mut T) }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        unsafe{ asm!("cpsie") }; // Enable interrupts
    }
}

