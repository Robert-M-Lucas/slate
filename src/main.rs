#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(slate::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;

mod sudoku_test;

use crate::sudoku_test::solution::Solution;
use crate::sudoku_test::solver::solve_backtracking;
use alloc::boxed::Box;
use alloc::collections::LinkedList;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use bootloader::{entry_point, BootInfo};
use core::hint::black_box;
use core::panic::PanicInfo;
use slate::lipsum::LipsumIterator;
use slate::memory::{translate_addr, BootInfoFrameAllocator};
use slate::other::{arbitrary_delay, arbitrary_short_delay};
use slate::{allocator, exit_qemu, hlt_loop, memory, print, println, QemuExitCode};
use x86_64::structures::paging::Page;
use x86_64::VirtAddr;
use slate::task::{keyboard, Task};
use slate::task::executor::Executor;
use slate::task::simple_executor::SimpleExecutor;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
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
    executor.run();

    println!("After");

    hlt_loop();
}

async fn main() {
    for word in LipsumIterator::new() {
        print!("{word} ");

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
