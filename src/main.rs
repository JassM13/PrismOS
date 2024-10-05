#![no_std]
#![no_main]

mod kernel;

use core::panic::PanicInfo;
use kernel::vga::text::print_something;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_something();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}