// main.rs

#![no_std] // Do not link against the Rust standard library
#![no_main] // Disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(ZY_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ZY_os::println;

// #[macro_use]
// mod vga_buffer;

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ZY_os::test_panic_handler(info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    
    loop {}
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    
    #[cfg(test)]
    test_main();
    
    loop {}
}
