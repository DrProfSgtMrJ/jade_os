#![no_std]
// conditionally enable no_main attribute
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
// otherwise function is ignored as the custom test framework 
// generates a main function that calls test_runner
#![reexport_test_harness_main = "test_main"] 

pub mod vga_buffer;
pub mod serial;
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

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        // Prints the function name
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        // -serial stdio set in cargo toml to redirect output to stdout
        serial_println!("[ok]");
    }
}

// Will run through the tests with the test_case label
// pub to be available to executables 
// no need for cfg(test)
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

/// Panic handler in test mode
/// Display is disabled in test-args (display = None)
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}