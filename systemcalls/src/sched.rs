use crate::error::Error;
use crate::nr::Syscalls;
use crate::syscall2;
use crate::unistd::Pid;

pub mod clone_flags {
    type CloneFlags = u32;
    pub const CLONE_VM: CloneFlags = 0x00000100;
    pub const CLONE_FS: CloneFlags = 0x00000200;
    pub const CLONE_FILES: CloneFlags = 0x00000400;
    pub const CLONE_SIGHAND: CloneFlags = 0x00000800;
    pub const CLONE_PIDFD: CloneFlags = 0x00001000;
    pub const CLONE_PTRACE: CloneFlags = 0x00002000;
    pub const CLONE_VFORK: CloneFlags = 0x00004000;
    pub const CLONE_PARENT: CloneFlags = 0x00008000;
    pub const CLONE_THREAD: CloneFlags = 0x00010000;
    pub const CLONE_NEWNS: CloneFlags = 0x00020000;
    pub const CLONE_SYSVSEM: CloneFlags = 0x00040000;
    pub const CLONE_SETTLS: CloneFlags = 0x00080000;
    pub const CLONE_PARENT_SETTID: CloneFlags = 0x00100000;
    pub const CLONE_CHILD_CLEARTID: CloneFlags = 0x00200000;
    pub const CLONE_DETACHED: CloneFlags = 0x00400000;
    pub const CLONE_UNTRACED: CloneFlags = 0x00800000;
    pub const CLONE_CHILD_SETTID: CloneFlags = 0x01000000;
    pub const CLONE_NEWCGROUP: CloneFlags = 0x02000000;
    pub const CLONE_NEWUTS: CloneFlags = 0x04000000;
    pub const CLONE_NEWIPC: CloneFlags = 0x08000000;
    pub const CLONE_NEWUSER: CloneFlags = 0x10000000;
    pub const CLONE_NEWPID: CloneFlags = 0x20000000;
    pub const CLONE_NEWNET: CloneFlags = 0x40000000;
    pub const CLONE_IO: CloneFlags = 0x80000000;
}

#[repr(C)]
#[derive(Default)]
pub struct CloneArgs {
    pub flags: u64,
    pub pidfd: u64,
    pub child_tid: u64,
    pub parent_tid: u64,
    pub exit_signal: u64,
    pub stack: u64,
    pub stack_size: u64,
    pub tls: u64,
    pub set_tid: u64,
    pub set_tid_size: u64,
    pub cgroup: u64,
}

#[inline(always)]
pub fn clone3(clone_args: CloneArgs) -> Result<Pid, Error> {
    let pid = syscall2(
        Syscalls::Clone3 as usize,
        core::ptr::addr_of!(clone_args) as usize,
        core::mem::size_of::<CloneArgs>(),
    ) as Pid;

    Ok(pid)
}
