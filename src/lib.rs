#![no_std]
#![feature(abi_x86_interrupt)]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(alloc_error_handler)]
#![feature(once_cell)]
#![feature(ready_macro)]

extern crate alloc;

use crate::allocator::init_heap;
use crate::memory::BootInfoFrameAllocator;
use bootloader::BootInfo;
use core::panic::PanicInfo;
use x86_64::VirtAddr;

pub mod allocator;
pub mod fs;
pub mod gdt;
pub mod interrupts;
pub mod io;
pub mod keyboard;
pub mod memory;
pub mod serial;
pub mod task;
pub mod test;
pub mod vga;

pub fn init(boot_info: &'static BootInfo) {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    init_heap(&mut mapper, &mut frame_allocator).expect("Heap allocation error");
}
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
