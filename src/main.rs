#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::panic::PanicInfo;
use systemcalls::println;
use systemcalls::sched::{clone3, clone_flags::*, CloneArgs};
use systemcalls::signal;
use systemcalls::unistd::{execve, exit, getpid, write};
use systemcalls::wait::waitpid;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panic: {:?}", info.message());
    exit(1);
    unreachable!();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    write(1, b"Hello world\n").expect("write failed: ");

    let cmd = b"/usr/bin/bash\0";
    let args = [&cmd.as_ptr(), core::ptr::null()];
    let env: [*const *const u8; 1] = [core::ptr::null()];

    println!("[Parent] my pid: {}", getpid());

    let clone_flags =
        CLONE_NEWUSER | CLONE_NEWUTS | CLONE_NEWPID | CLONE_NEWNS | CLONE_NEWIPC | CLONE_NEWNET;

    let clone_args = CloneArgs {
        exit_signal: signal::SIGCHLD as u64,
        flags: clone_flags as u64,
        ..Default::default()
    };
    let pid = clone3(clone_args).expect("clone3 failed: ");

    if pid != 0 {
        println!("[Parent] child pid: {}", pid);
        waitpid(pid as i32, 0).expect("waitpid failed: ");
    } else {
        println!("[Child] process start");
        execve(&cmd[..], &args[..], &env[..]).expect("execve failed: ");
    }

    exit(0);

    unreachable!();
}
