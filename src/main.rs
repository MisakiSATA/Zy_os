// main.rs

#![no_std] // Do not link against the Rust standard library
#![no_main] // Disable all Rust-level entry points

use core::panic::PanicInfo;

// This function will be called upon a panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

mod vga_buffer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER
        .lock()
        .write_str("Hello again")
        .unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        ", some numbers: {} {}",
        42,
        1.337
    )
    .unwrap();
    loop {}
}