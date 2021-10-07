use crate::error::Error;
use crate::fcntl::o_flag::OFlag;
use crate::nr::Syscalls;
use crate::unistd::RawFd;
use crate::{syscall3, syscall_ret};

pub fn open(path: &str, flag: OFlag, mode: i32) -> Result<RawFd, Error> {
    let ret = syscall3(
        Syscalls::Open as usize,
        path.as_ptr() as usize,
        flag as usize,
        mode as usize,
    );

    syscall_ret(ret).map(|ret| ret as i32)
}

pub mod o_flag {
    pub type OFlag = usize;
    pub const O_RDONLY: OFlag = 0x00;
    pub const O_WRONLY: OFlag = 0x01;
    pub const O_RDWR: OFlag = 0x02;
    pub const O_CREAT: OFlag = 0x0100;
    pub const O_EXCL: OFlag = 0x0200;
    pub const O_NOCTTY: OFlag = 0x0400;
    pub const O_TRUNC: OFlag = 0x01000;
    pub const O_APPEND: OFlag = 0x02000;
    pub const O_NONBLOCK: OFlag = 0x04000;
    pub const O_DSYNC: OFlag = 0x010000;
    pub const O_SYNC: OFlag = 0x04010000;
    pub const O_ASYNC: OFlag = 0x020000;
    pub const O_LARGEFILE: OFlag = 0x0100000;
    pub const O_CLOEXEC: OFlag = 0x02000000;
    pub const O_DIRECT: OFlag = 0x040000;
    pub const O_DIRECTORY: OFlag = 0x0200000;
    pub const O_NOATIME: OFlag = 0x01000000;
    pub const O_NOFOLLOW: OFlag = 0x0400000;
    pub const O_PATH: OFlag = 0x010000000;
    pub const O_TMPFILE: OFlag = 0x020200000;
}

/*
#define O_ACCMODE          0003
#define O_RDONLY             00
#define O_WRONLY             01
#define O_RDWR               02
# define O_CREAT        flag   0100 /* Not fcntl.  */
# define O_EXCL            0200 /* Not fcntl.  */
# define O_NOCTTY          0400 /* Not fcntl.  */
# define O_TRUNC          01000 /* Not fcntl.  */
# define O_APPEND         02000
# define O_NONBLOCK       04000
# define O_SYNC        04010000
# define O_ASYNC         020000
# define __O_LARGEFILE  0100000
# define __O_DIRECTORY  0200000
# define __O_NOFOLLOW   0400000
# define __O_CLOEXEC   02000000
# define __O_DIRECT      040000
# define __O_NOATIME   01000000
# define __O_PATH     010000000
# define __O_DSYNC       010000
# define __O_TMPFILE   (020000000 | __O_DIRECTORY)
#endif
*/
