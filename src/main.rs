// main.rs

#![no_std] // Do not link against the Rust standard library
#![no_main] // Disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");
    
    #[cfg(test)]
    test_main();
    
    loop {}
}


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    println!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}