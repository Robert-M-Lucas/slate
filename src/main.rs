#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(slate::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;

mod sudoku_test;

use alloc::rc::Rc;
use alloc::vec;
use crate::sudoku_test::solution::Solution;
use crate::sudoku_test::solver::solve_backtracking;
use bootloader::{entry_point, BootInfo};
use core::hint::black_box;
use core::panic::PanicInfo;
use x86_64::structures::paging::Page;
use slate::memory::{translate_addr, BootInfoFrameAllocator};
use slate::other::arbitrary_delay;
use slate::{allocator, exit_qemu, hlt_loop, memory, println, QemuExitCode};
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    slate::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    // main();

    hlt_loop();
}

fn main() {
    println!("Hello World{}", "!");

    println!("Loading sudoku");

    let problem = Solution::load_string(include_str!("sudoku.txt"));

    println!("Loaded");
    println!("{}", problem);

    println!("Solving");
    let solution = solve_backtracking(problem.clone());
    if let Some(solution) = solution {
        println!("{}", solution);
    } else {
        println!("No solution found");
    }

    println!("Solving 1000");
    for _ in 0..1000 {
        let solution = solve_backtracking(problem.clone());
        black_box(solution);
    }

    println!("Done");
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
