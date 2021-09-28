#![feature(asm)]
#![no_std]

pub mod errno;
pub mod nr;
use nr::Syscalls;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

struct MyWriter;

impl fmt::Write for MyWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(1, s.as_bytes());
        Ok(())
    }
}

use core::fmt;
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = MyWriter {};
    writer.write_fmt(args).unwrap();
}

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
pub fn execve(cmd: &[u8], args: &[*const *const u8], env: &[*const *const u8]) -> i32 {
    syscall3(
        Syscalls::Execve as usize,
        cmd.as_ptr() as usize,
        args.as_ptr() as usize,
        env.as_ptr() as usize,
    ) as i32
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
