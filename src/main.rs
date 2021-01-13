#![no_std]
#![no_main]
use core::panic::PanicInfo;


// fn main() {
// }

#[no_mangle] //forces it to be called "_start"
pub extern "C" fn _start() -> ! {
    //entry point for our program
    loop {}
}

#[panic_handler]
//for when things go bad
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
