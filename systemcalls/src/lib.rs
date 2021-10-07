#![feature(asm)]
#![no_std]

pub mod error;
pub mod fcntl;
pub mod io;
pub mod nr;
pub mod sched;
pub mod signal;
pub mod unistd;
pub mod wait;

use error::Error;
pub use io::_print;

#[inline(always)]
fn syscall_ret(ret: usize) -> Result<usize, Error> {
    if ret < 0xFFFFFFFFFFFF0000 {
        Ok(ret)
    } else {
        let errno = !(ret) + 0x0000000000000001;
        Err(Error {
            errno: errno.into(),
        })
    }
}

#[inline(always)]
pub fn syscall0(arg0: usize) -> usize {
    let ret: usize;
    unsafe {
        asm!(
            "syscall",
            in("rax") arg0,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
        );
    }
    ret
}

#[inline(always)]
pub fn syscall1(arg0: usize, arg1: usize) -> usize {
    let ret: usize;
    unsafe {
        asm!(
            "syscall",
            in("rax") arg0,
            in("rdi") arg1,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
        );
    }
    ret
}

#[inline(always)]
pub fn syscall2(arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret: usize;
    unsafe {
        asm!(
            "syscall",
            in("rax") arg0,
            in("rdi") arg1,
            in("rsi") arg2,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
        );
    }
    ret
}

#[inline(always)]
pub fn syscall3(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    let ret: usize;
    unsafe {
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
    }
    ret
}

#[inline(always)]
pub fn syscall4(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    let ret: usize;
    unsafe {
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
    }
    ret
}

#[inline(always)]
pub fn syscall5(
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> usize {
    let ret: usize;
    unsafe {
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
    }
    ret
}

#[inline(always)]
pub fn syscall6(
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
) -> usize {
    let ret: usize;
    unsafe {
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
    }
    ret
}
