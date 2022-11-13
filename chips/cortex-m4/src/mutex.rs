use cortex_m::interrupt::disable;
use cortex_m::interrupt::enable;
use core::marker::{Sync, PhantomData};
use kernel::mutex::Mux;

pub struct Mutex<T>{
    pub data: PhantomData<T>
}

impl<T> Mux for Mutex<T> {
    type Wrapped = T;
    
    fn lock(&self, obj: &T) -> &mut T {
        unsafe { 
            disable(); // Disable interrupts
            &mut *(obj as *const T as *mut T)
        } 
    }
    fn unlock(&self) {
        unsafe {
            enable(); // Enable interrupts
        }
    }
}

unsafe impl<T> Sync for Mutex<T> {}

