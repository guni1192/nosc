use crate::unistd::write;

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
        write(1, s.as_bytes()).expect("write failed: ");
        Ok(())
    }
}

use core::fmt;

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = MyWriter {};
    writer.write_fmt(args).unwrap();
}
