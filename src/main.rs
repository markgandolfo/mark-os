#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mark_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use mark_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello world{}", "!");

    mark_os::init();

    // page fault
    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table);

    // Double fault execption
    // unsafe { *(0xdeadbeef as *mut u8) = 42 };

    // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3();

    // stack overflow
    // fn stack_overflow() {
    //     stack_overflow();
    // }
    // stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    mark_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    mark_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mark_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
