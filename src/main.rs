#![no_std]
#![no_main]

use core::panic::PanicInfo;
use systemcalls::{execve, exit, write};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    write(1, b"Hello world");

    let cmd = b"/usr/bin/bash\0";
    let args = [&cmd.as_ptr(), core::ptr::null()];
    let env: [*const *const u8; 1] = [core::ptr::null()];

    execve(&cmd[..], &args[..], &env[..]);

    exit(0);

    unreachable!();
}
