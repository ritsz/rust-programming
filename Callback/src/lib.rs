use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn callback(a: *const c_char) {
    println!("I'm called from C with value {:?}", unsafe {CStr::from_ptr(a).to_str().unwrap()} );
}

