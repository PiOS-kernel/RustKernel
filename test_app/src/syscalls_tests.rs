use kernel::syscalls::create_task;


fn foo(pippo : *mut u8) -> ! {
    loop {
        
    }
}

#[test_case]
fn test_create_task() {
    create_task(foo, 0 as *mut u8, 0);
}

#[test_case]
fn test_systick_handler() {
    // systick initializatoin
    

    //loop { };
}