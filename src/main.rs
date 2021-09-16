#![no_std]
#![no_main]

use core::panic::PanicInfo;
use systemcalls::{exit, getpid, println, write};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    write(1, b"Hello world!\n");
    write(1, b"getpid\n");

    println!("pid: {}", getpid());

    exit(0);

    unreachable!();
}
