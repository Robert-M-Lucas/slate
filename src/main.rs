#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(slate::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use slate::lipsum::LipsumIterator;
use slate::memory::BootInfoFrameAllocator;
use slate::task::executor::Executor;
use slate::task::{keyboard, Task};
use slate::{allocator, hlt_loop, memory, print, println, serial_println};
use x86_64::VirtAddr;
use slate::other::arbitrary_delay;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    serial_println!("Serial hello, world!");

    slate::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    println!("Before");

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses())); // new
    // executor.spawn(Task::new(main()));
    executor.run();

    println!("After");

    hlt_loop();
}

async fn main() {
    for word in LipsumIterator::new() {
        print!("{word} ");
        arbitrary_delay();
    }
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
