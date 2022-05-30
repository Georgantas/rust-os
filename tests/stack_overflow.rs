
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rust_os::serial_print;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");

    rust_os::gdt::init();
    
    init_test_idt();

    stack_overflow();

    panic!("Execution continued after stack overflow.")
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    // prevent tail recursion optimization
    volatile::Volatile::new(0).read();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}


