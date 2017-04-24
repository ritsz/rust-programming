#[no_mangle]
pub extern "C" fn callback(a: i32) {
    println!("I'm called from C with value {0}", a);
}

