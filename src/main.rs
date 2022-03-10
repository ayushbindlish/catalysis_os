// To write an operating system kernel, we need code that does not depend on and
// operating system.

// Disable automatic linking of the standard library
#![no_std] // do not link the rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

// panic_handler attribute defines the function
// that the compiler invokes should a panic occur
#[panic_handler] /// this function is called on panic
fn panic(_info: &PanicInfo) -> ! {
    // returns "never" type using "!" i.e. it does not return
    // marked as diverging function by returning !
    loop {}
}

#[no_mangle] // do not mangle the name of this function
pub extern "C" fn _start() -> ! {
    loop {}
}
