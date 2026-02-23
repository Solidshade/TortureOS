#![no_std] // dont link rust standard library
#![no_main] // disable all rust-level entry points
use core::panic::PanicInfo;

//this function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)] // dont mangle the name of this function, since it is called by the bootloader
pub extern "C" fn _start() -> ! {
    // this func is the entry point, since the linker loooks for a function names _start by
    // default, we can change this name by changing the linker script, but we will stick with
    // _start for simplicity.
    loop {}
}
