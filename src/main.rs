#![no_std] // dont link rust std lib
#![no_main] // disable rust-level entry points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!"; // hello_world("print")

#[unsafe(no_mangle)] // dont mangle name of function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function named
    // _start by default, can change by changing linker scripts but will stick with
    // _start for simplicity( and sloth)
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
