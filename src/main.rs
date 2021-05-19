#![feature(asm)]
#![no_std]
#![no_main]
mod vga_buffer;
use core::panic::PanicInfo;
// use fmt::Write

static HELLO: &[u8] = b"Hello You!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("this is a panic");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
