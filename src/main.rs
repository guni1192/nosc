#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::panic::PanicInfo;
use systemcalls::error::Error;
use systemcalls::fcntl::{o_flag, open};
use systemcalls::println;
use systemcalls::sched::{clone3, clone_flags::*, CloneArgs};
use systemcalls::signal;
use systemcalls::unistd::{execve, exit, getpid, mkdir, sethostname, RawFd};
use systemcalls::wait::waitpid;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panic: {:?}", info.message());
    exit(1);
    unreachable!();
}

fn cgroup_open() -> Result<RawFd, Error> {
    let path = "/sys/fs/cgroup/nosc\0";
    let flag = o_flag::O_RDONLY | o_flag::O_DIRECTORY;
    let _ = mkdir(path, 0o644);

    open(path, flag, 0o644)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let cmd = b"/usr/bin/bash\0";
    let args = [&cmd.as_ptr(), core::ptr::null()];
    let env: [*const *const u8; 1] = [core::ptr::null()];

    let fd = cgroup_open().expect("failed to open cgroup directory");
    println!("cgroup dir fd: {}", fd);

    println!("[Parent] my pid: {}", getpid());

    let clone_flags =
        CLONE_INTO_CGROUP | CLONE_NEWUTS | CLONE_NEWPID | CLONE_NEWNS | CLONE_NEWIPC | CLONE_NEWNET;

    let clone_args = CloneArgs {
        exit_signal: signal::SIGCHLD as u64,
        flags: clone_flags as u64,
        cgroup: fd as u64,
        ..Default::default()
    };

    let pid = clone3(clone_args).expect("clone3 failed: ");

    if pid != 0 {
        println!("[Parent] child pid: {}", pid);
        waitpid(pid as i32, 0).expect("waitpid failed: ");
    } else {
        sethostname("nosc-container").expect("failed to sethostname");
        execve(&cmd[..], &args[..], &env[..]).expect("execve failed: ");
    }

    exit(0);

    unreachable!();
}
