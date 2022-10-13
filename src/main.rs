#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use x86_64::registers::control::Cr3;
use rust_os::println;

// No mangle ensure that the Rust compiler really
// outputs a function with the name _start.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world!");

    rust_os::init();

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    #[cfg(test)]
    test_main();

    println!("No crash!");

    rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1 + 1, 2);
}
