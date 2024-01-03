// NOT externally visible
fn local() {
    println!("inside preload::local() now");
}

// NOT externally visible, but allows us to call the shim sleep()
// function from the Rust code.
extern "C" {
    fn sleep(x: u32) -> u32;
}

// externally visible, when the preloaded app calls time we will intercept it.
#[no_mangle]
pub extern "C" fn time(ptr: *mut core::ffi::c_void) -> core::ffi::c_uint {
    println!("inside preload::time({ptr:p}) now");

    println!("calling shim::sleep(3) now to help service the syscall.");
    let rv = unsafe { sleep(3) };
    println!("shim::sleep(3) returned {rv}");

    local();
    123456
}

// if we want shim.c to be able to call this function, we have to add
// #[no_mangle], but then we may get unintended interceptions if the app
// we are preloading also calls external().
// pub extern "C" fn external() {
//     println!("inside preload::external() now");
// }
