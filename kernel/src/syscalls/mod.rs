pub enum SysCallID {
    CREATE_TASK = 1,
}

pub fn kcreate_task(code: fn(*mut u8), args: *mut u8) {
    
}