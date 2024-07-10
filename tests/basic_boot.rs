#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mark_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use mark_os::println;

// as this is a standalone executable we need to nostd, nomain, etc.
// as well as provide an entry point in for our program.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mark_os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
