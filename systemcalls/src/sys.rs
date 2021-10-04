use crate::errno::Error as SysError;
use crate::nr::Syscalls;
use crate::{syscall0, syscall1, syscall3, syscall5, syscall6};

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

type CloneFlags = u32;

#[rustfmt::skip]
pub const CLONE_VM: CloneFlags              = 0x00000100;
#[rustfmt::skip]
pub const CLONE_FS: CloneFlags              = 0x00000200;
#[rustfmt::skip]
pub const CLONE_FILES: CloneFlags           = 0x00000400;
#[rustfmt::skip]
pub const CLONE_SIGHAND: CloneFlags         = 0x00000800;
#[rustfmt::skip]
pub const CLONE_PIDFD: CloneFlags           = 0x00001000;
#[rustfmt::skip]
pub const CLONE_PTRACE: CloneFlags          = 0x00002000;
#[rustfmt::skip]
pub const CLONE_VFORK: CloneFlags           = 0x00004000;
#[rustfmt::skip]
pub const CLONE_PARENT: CloneFlags          = 0x00008000;
#[rustfmt::skip]
pub const CLONE_THREAD: CloneFlags          = 0x00010000;
#[rustfmt::skip]
pub const CLONE_NEWNS: CloneFlags           = 0x00020000;
#[rustfmt::skip]
pub const CLONE_SYSVSEM: CloneFlags         = 0x00040000;
#[rustfmt::skip]
pub const CLONE_SETTLS: CloneFlags          = 0x00080000;
#[rustfmt::skip]
pub const CLONE_PARENT_SETTID: CloneFlags   = 0x00100000;
#[rustfmt::skip]
pub const CLONE_CHILD_CLEARTID: CloneFlags  = 0x00200000;
#[rustfmt::skip]
pub const CLONE_DETACHED: CloneFlags        = 0x00400000;
#[rustfmt::skip]
pub const CLONE_UNTRACED: CloneFlags        = 0x00800000;
#[rustfmt::skip]
pub const CLONE_CHILD_SETTID: CloneFlags    = 0x01000000;
#[rustfmt::skip]
pub const CLONE_NEWCGROUP: CloneFlags       = 0x02000000;
#[rustfmt::skip]
pub const CLONE_NEWUTS: CloneFlags          = 0x04000000;
#[rustfmt::skip]
pub const CLONE_NEWIPC: CloneFlags          = 0x08000000;
#[rustfmt::skip]
pub const CLONE_NEWUSER: CloneFlags         = 0x10000000;
#[rustfmt::skip]
pub const CLONE_NEWPID: CloneFlags          = 0x20000000;
#[rustfmt::skip]
pub const CLONE_NEWNET: CloneFlags          = 0x40000000;
#[rustfmt::skip]
pub const CLONE_IO: CloneFlags              = 0x80000000;

#[repr(C)]
pub struct CloneArgs {
    flags: u64,
    pidfd: u64,
    child_tid: u64,
    parent_tid: u64,
    exit_signal: u64,
    stack: u64,
    stack_size: u64,
    tls: u64,
    set_tid: u64,
    set_tid_size: u64,
}

pub fn hello(_: *mut usize) -> i64 {
    write(1, b"hoge\n");
    0
}

#[inline(always)]
pub fn clone() -> Result<(), SysError> {
    /*
        pub unsafe extern "C" fn clone(
        cb: extern "C" fn(_: *mut c_void) -> c_int,
        child_stack: *mut c_void,
        flags: c_int,
        arg: *mut c_void,
         ...
    ) -> c_int
    */

    // the kernel entry is:
    // int clone (long flags, void *child_stack, pid_t *parent_tid,
    //            pid_t *child_tid, void *tls);

    let stack_size = 1024 * 1024;
    let stack = mmap(
        core::ptr::null(),
        stack_size,
        PROT_WRITE | PROT_READ,
        MAP_PRIVATE | MAP_ANONYMOUS | MAP_STACK,
        -1,
        0,
    );

    syscall5(
        Syscalls::Clone as usize,
        (SIGCHLD) as usize, // long flags
        stack as usize,     // void *child_stack
        0usize,             // pid_t *parent_tid
        0usize,             // pid_t *child_tid
        0usize,             // void *tls
    );

    Ok(())
}

#[inline(always)]
pub fn clone3() -> Result<(), SysError> {
    Ok(())
}

pub const SIGHUP: u32 = 1;
pub const SIGINT: u32 = 2;
pub const SIGQUIT: u32 = 3;
pub const SIGILL: u32 = 4;
pub const SIGABRT: u32 = 6;
pub const SIGEMT: u32 = 7;
pub const SIGFPE: u32 = 8;
pub const SIGKILL: u32 = 9;
pub const SIGSEGV: u32 = 11;
pub const SIGPIPE: u32 = 13;
pub const SIGALRM: u32 = 14;
pub const SIGTERM: u32 = 15;
pub const SIGCHLD: u32 = 17;

pub fn mmap(
    addr: *const u8,
    length: usize,
    prot: i32,
    flags: i32,
    fd: i32,
    offset: i32,
) -> *const u8 {
    syscall6(
        Syscalls::Mmap as usize,
        addr as usize,
        length as usize,
        prot as usize,
        flags as usize,
        fd as usize,
        offset as usize,
    ) as *const u8
}

pub const MAP_FILE: i32 = 0x0000;
pub const MAP_SHARED: i32 = 0x0001;
pub const MAP_PRIVATE: i32 = 0x0002;
pub const MAP_FIXED: i32 = 0x0010;
pub const MAP_ANONYMOUS: i32 = 0x0020;
pub const MAP_STACK: i32 = 0x20000;

pub const PROT_NONE: i32 = 0;
pub const PROT_READ: i32 = 1;
pub const PROT_WRITE: i32 = 2;
pub const PROT_EXEC: i32 = 4;