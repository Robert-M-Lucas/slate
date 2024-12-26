#![no_std]
#![no_main]

mod vga_buffer;

use core::fmt::Write;
use core::hint::black_box;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world!");

    for x in 0..10_000_000 {
        black_box(x);
    }

    println!("After delay");

    loop {}
}

