#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points

mod vga_buffer;

use core::panic::PanicInfo;

/// Called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // Function is entry point, since the linker looks for a funciton named
    // `_start` by default.
    panic!("Some panic message");

}
