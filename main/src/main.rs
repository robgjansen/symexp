fn main() {
    println!("main: calling libc::time(0x0)");
    let rv = unsafe { libc::time(std::ptr::null_mut()) };
    println!("main: libc::time(0x0) result: {rv}");

    println!("main: calling lib::sleep(3)");
    let rv = unsafe { libc::sleep(3) };
    println!("main: libc::sleep(3) result: {rv}");
}
