#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points

mod vga_buffer;

use core::panic::PanicInfo;

/// Called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // Function is entry point, since the linker looks for a funciton named
    // `_start` by default.
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
     write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    loop {}
}
