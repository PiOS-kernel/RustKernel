use core::ops::{Deref, DerefMut};
use cortex_m::interrupt::{enable, disable};
use core::marker::Sync;

pub struct Mutex<T> {
    inner: T,
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Mutex<T> {
    pub const fn new(inner: T) -> Self {
        Self {inner: inner}
    }

    pub fn lock(&self) -> MutexGuard<T> {
        unsafe {
            disable(); // Disable interrupts
            MutexGuard::new(self)
        } 
    }
}

unsafe impl<T> Sync for Mutex<T> {}

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
        unsafe{ enable() }; // Enable interrupts
    }
}
