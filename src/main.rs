#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(minimal_os::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;

use minimal_os::*;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use minimal_os::allocator::init_heap;
use minimal_os::keyboard::print_keypresses;
use minimal_os::memory::BootInfoFrameAllocator;
use minimal_os::task::executor::Executor;
use minimal_os::task::Task;
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    init_heap(&mut mapper, &mut frame_allocator).expect("Heap allocation error");

    let mut executor = Executor::new();

    executor.spawn(Task::new(print_keypresses()));

    #[cfg(test)]
    test_main();
    executor.run()
}
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test::test_panic_handler(info)
}
