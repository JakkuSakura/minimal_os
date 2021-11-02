#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(minimal_os::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;

use minimal_os::*;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use minimal_os::keyboard::print_keypresses;
use minimal_os::task::executor::Executor;
use minimal_os::task::Task;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);
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
