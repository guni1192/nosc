use crate::error::Error;
use crate::nr::Syscalls;
use crate::unistd::RawFd;
use crate::{syscall3, syscall_ret};

pub fn open(path: &str, flag: i32, mode: i32) -> Result<RawFd, Error> {
    let ret = syscall3(
        Syscalls::Open as usize,
        path.as_ptr() as usize,
        flag as usize,
        mode as usize,
    );

    syscall_ret(ret).map(|ret| ret as i32)
}
