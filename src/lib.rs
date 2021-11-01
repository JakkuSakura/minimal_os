#![no_std]
#![feature(abi_x86_interrupt)]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod test;
pub mod vga;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
