#![no_std]
#![no_main]
#![allow(unused)]

use core::panic::PanicInfo;
mod hal;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unimplemented!()
}