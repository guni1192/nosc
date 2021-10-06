use crate::error::Error;
use crate::nr::Syscalls;
use crate::{syscall0, syscall1, syscall3, syscall_ret};

pub type Pid = i32;
pub type RawFd = i32;

#[inline(always)]
pub fn write(fd: usize, buf: &[u8]) -> Result<usize, Error> {
    let ret = syscall3(
        Syscalls::Write as usize,
        fd,
        buf.as_ptr() as usize,
        buf.len(),
    );

    syscall_ret(ret)
}

#[inline(always)]
pub fn getpid() -> Pid {
    syscall0(Syscalls::Getpid as usize) as Pid
}

#[inline(always)]
pub fn exit(status: i32) {
    syscall1(Syscalls::Exit as usize, status as usize);
}

#[inline(always)]
pub fn execve(
    cmd: &[u8],
    args: &[*const *const u8],
    env: &[*const *const u8],
) -> Result<(), Error> {
    let ret = syscall3(
        Syscalls::Execve as usize,
        cmd.as_ptr() as usize,
        args.as_ptr() as usize,
        env.as_ptr() as usize,
    );

    syscall_ret(ret)?;

    Ok(())
}
