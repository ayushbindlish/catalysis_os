// To write an operating system kernel, we need code that does not depend on and
// operating system.
// Build command "cargo rustc -- -C link-arg=-nostartfiles"


// Disable linking of the standard library
#![no_std]

#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
