pub fn mmap(
    addr: *const u8,
    length: usize,
    prot: i32,
    flags: i32,
    fd: i32,
    offset: i32,
) -> *const usize {
    syscall6(
        Syscalls::Mmap as usize,
        addr as usize,
        length as usize,
        prot as usize,
        flags as usize,
        fd as usize,
        offset as usize,
    ) as *const usize
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
