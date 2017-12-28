extern crate libc;

use libc::{c_int, size_t};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::mem;


#[link(name = "wrapper")]
extern {
	fn create_array(array: *mut u32, num: u32); 
}

#[no_mangle]
pub extern "C" fn callback(a: *const c_char) {
	
	unsafe {
		let ptr: *mut u32 = libc::malloc(5 * mem::size_of::<u32>() as libc::size_t) as *mut u32;
		assert!(!ptr.is_null());

		println!("{:?}", ptr);
		create_array(&mut *ptr, 5);

		let mut x = 0;
		let mut itr = ptr;

		while x < 5 {
			x = x + 1;
			println!("{:?}", *itr);
			itr = ptr.offset(1);
		}

		libc::free(ptr as *mut libc::c_void);
	}

    println!("I'm called from C with a value of {:?}", unsafe {CStr::from_ptr(a).to_str().unwrap()} );
}

