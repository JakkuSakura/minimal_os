#![no_std]
#![no_main]

use core::panic::PanicInfo;

use minimal_os::println;

fn print_message() {
    println!("Hello World!\nYou are my world!");
}
/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_message();
    panic!("I like panic!");
}
