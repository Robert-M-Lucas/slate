#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(slate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use slate::other::arbitrary_delay;
use slate::{exit_qemu, hlt_loop, println, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    slate::init();


    #[cfg(test)]
    test_main();

    main();

    hlt_loop();
}

fn main() {
    println!("Hello World{}", "!");

    arbitrary_delay();

    println!("{}", include_str!("long_text.txt"));
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    slate::test_panic_handler(info)
}