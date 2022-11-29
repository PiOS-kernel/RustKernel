set auto-load safe-path
file ./target/thumbv7em-none-eabi/debug/deps/test_app-7dc3b757d6e00c97
target remote :3333
break test_task_switch
break HardFault
continue
# break create_task
# break SVCall
# continue
# disassemble
# continue
# si
# si
# si
# disassemble