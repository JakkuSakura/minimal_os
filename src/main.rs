#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(minimal_os::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use minimal_os::*;

use core::panic::PanicInfo;

fn print_message() {
    println!("Hello World!\nYou are my world!");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    print_message();
    
    #[cfg(test)]
    test_main();
    panic!("I like panic!");
}
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test::test_panic_handler(info)
}