#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
// otherwise function is ignored as the custom test framework 
// generates a main function that calls test_runner
#![reexport_test_harness_main = "test_main"] 

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;

// Want to make sure we have different exit codes from the default codes of QEMU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10, // Requires setting the test-success-exit-code = 33 sicne (0x10 << 1) | 1 = 33
    Failed = 0x11,
}

// isa-debug-exit: When value is written to the I/O port at iobase
// it causes QEMU to exist with exit status: (value << 1) | 1
// 0 => (0 << 1) | 1 = 1
// 1 => (1 << 1) | 1 = 3
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    // Writing to I/O port can generally result in arbitrary behavior
    unsafe {
        // Generally an unused port on x86 IO bus
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

/// Called on panic.
/// for not in test mode
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// Panic handler in test mode
/// Display is disabled in test-args (display = None)
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // Function is entry point, since the linker looks for a funciton named
    // `_start` by default.
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

// Will run through the tests with the test_case label
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    serial_println!("trivial assertion... ");
    assert_eq!(0, 1);
    serial_println!("[ok]"); // -serial stdio set in cargo toml to redirect output to stdout
}
