#![feature(asm)]

pub mod nr;
use nr::Syscalls;

pub fn write(fd: usize, buf: &[u8]) {
    unsafe {
        syscall3(
            Syscalls::SYS_write as usize,
            fd,
            buf.as_ptr() as usize,
            buf.len(),
        )
    };
}

#[inline(always)]
pub unsafe fn syscall0(arg0: usize) -> isize {
    let ret: isize;
    asm!(
        "syscall",
        in("rax") arg0,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall1(arg0: usize, arg1: usize) -> isize {
    let ret: isize;
    asm!(
        "syscall",
        in("rax") arg0,
        in("rdi") arg1,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall2(arg0: usize, arg1: usize, arg2: usize) -> isize {
    let ret: isize;
    asm!(
        "syscall",
        in("rax") arg0,
        in("rdi") arg1,
        in("rsi") arg2,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall3(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> isize {
    let ret: isize;
    asm!(
        "syscall",
        in("rax") arg0,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall4(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> isize {
    let ret: isize;
    asm!(
        "syscall",
        in("rax") arg0,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall5(
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> isize {
    let ret: isize;
    asm!(
        "syscall",
        in("rax") arg0,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        in("r8") arg5,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall6(
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
) -> isize {
    let ret: isize;
    asm!(
        "syscall",
        in("rax") arg0,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        in("r8") arg5,
        in("r9") arg6,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
    );
    ret
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
