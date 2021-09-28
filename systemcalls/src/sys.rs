use crate::errno::Error as SysError;
use crate::nr::Syscalls;
use crate::{syscall0, syscall1, syscall3};

#[inline(always)]
pub fn write(fd: usize, buf: &[u8]) -> usize {
    syscall3(
        Syscalls::Write as usize,
        fd,
        buf.as_ptr() as usize,
        buf.len(),
    )
}

#[inline(always)]
pub fn getpid() -> i32 {
    syscall0(Syscalls::Getpid as usize) as i32
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
) -> Result<(), SysError> {
    let ret = syscall3(
        Syscalls::Execve as usize,
        cmd.as_ptr() as usize,
        args.as_ptr() as usize,
        env.as_ptr() as usize,
    );

    if ret < 0xFFFFFFFFFFFF0000 {
        Ok(())
    } else {
        let errno = !(ret) + 0x0000000000000001;
        Err(SysError {
            errno: errno.into(),
        })
    }
}
