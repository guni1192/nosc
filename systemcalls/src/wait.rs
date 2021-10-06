use crate::error::Error;
use crate::nr::Syscalls;
use crate::unistd::Pid;
use crate::{syscall4, syscall_ret};

#[inline(always)]
pub fn waitpid(pid: Pid, options: i32) -> Result<(), Error> {
    let status = 0;

    let ret = syscall4(
        Syscalls::Wait4 as usize,
        pid as usize,
        status as usize,
        options as usize,
        0,
    );

    syscall_ret(ret)?;
    Ok(())
}
