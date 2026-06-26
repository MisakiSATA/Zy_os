// main.rs

#![no_std] // Do not link against the Rust standard library
#![no_main] // Disable all Rust-level entry points

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
    panic!("Some panic message");
    
    loop {}
}

