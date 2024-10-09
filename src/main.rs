#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(jade_os::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use jade_os::println;


#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // Function is entry point, since the linker looks for a funciton named
    // `_start` by default.
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

/// Called on panic.
/// for not in test mode
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    jade_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}