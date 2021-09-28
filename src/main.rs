#![no_std]
#![no_main]

use core::panic::PanicInfo;
use systemcalls::println;
use systemcalls::sys::{execve, exit, write};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{:?}", info);
    exit(1);
    unreachable!();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    write(1, b"Hello world\n");

    let cmd = b"/usr/bin/bash\0";
    let args = [&cmd.as_ptr(), core::ptr::null()];
    let env: [*const *const u8; 1] = [core::ptr::null()];

    execve(&cmd[..], &args[..], &env[..]).expect("execve failed: ");

    exit(0);

    unreachable!();
}
