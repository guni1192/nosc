use systemcalls::write;

fn main() {
    let buf = b"Hello from asm!\n";
    write(1, buf);
}
