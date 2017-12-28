extern crate libc;

use libc::{c_int, size_t};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::mem;
use std::ops::{Index, IndexMut};
use std::marker::PhantomData;


#[link(name = "wrapper")]
extern {
	fn create_array(array: *mut u32, num: u32); 
}

struct CArrayIter<'a> {
	curr: usize,
	data: &'a CArrayInt<'a>,
	phantom: PhantomData<&'a u32>,
}

/* The iterator returns a non mutable reference from the buffer for each element. */
impl<'a> Iterator for CArrayIter<'a> {
	type Item = &'a u32;

	fn next(&mut self) -> Option<Self::Item> {
		if self.curr < self.data.size {
			let dat = unsafe { self.data.ptr.offset(self.curr as isize) };
			self.curr = self.curr + 1;
			unsafe{dat.as_ref()}
		} else{
			None
		}
	}
}

struct CArrayInt<'a> {
	ptr: *mut  u32,
	size: usize,
	phantom: PhantomData<&'a u32>,
}

impl<'a>  CArrayInt<'a> {
	fn new(size: usize) -> CArrayInt<'a> {
		
		let iptr: *mut u32 = unsafe { libc::malloc(size * mem::size_of::<u32>() as libc::size_t) as *mut u32 };
		assert!(!iptr.is_null());

		unsafe {
			create_array(&mut *iptr, size as u32);
		}

		CArrayInt {
			ptr: iptr,
			size: size,
			phantom: PhantomData,
		}
	}

	/* Create the iterator object */
	fn iter(&self) -> CArrayIter {
		println!("ITER");
		CArrayIter {
			curr: 0,
			data: self,
			phantom: PhantomData,
		}
	}
}

/* 
 * NOTE: The IntoIterator is implemented on a reference of CArrayInt, so that we are allowed to do 
 * for x in &buf(). If reference is not added, we can only do for x in buf, which would move the buffer.
 */
impl<'a> IntoIterator for &'a CArrayInt<'a> {
	type Item = &'a u32;
	type IntoIter = CArrayIter<'a>;

	/* Define the into_iterator function that tells how the iterator of this struct is like */
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a> Index<u32> for CArrayInt<'a>{
	type Output = u32;

	fn index(&self, index: u32) -> &Self::Output {
		unsafe {
			self.ptr.offset(index as isize).as_ref().unwrap()
		}
	}
}

/* Index operator that return mutable references, so that index operator can update the buffers */
impl<'a> IndexMut<u32> for CArrayInt<'a>{
	fn index_mut(&mut self, index: u32) -> &mut u32 {
		unsafe {
			self.ptr.offset(index as isize).as_mut().unwrap()
		}
	}
}

/* Destructor that frees up the memory given by libc */
impl<'a> Drop for CArrayInt<'a> {
	fn drop(&mut self) {
		println!("Dropping!");
		unsafe {
			libc::free(self.ptr as *mut libc::c_void);
		}
	}
}


#[no_mangle]
pub extern "C" fn callback(a: *const c_char) {

	let mut ptr = CArrayInt::new(5);

	for i in 0..5 {
		println!("{:?}", ptr[i]);
		ptr[i] = ptr[i]*ptr[i];
	}

	for x in &ptr {
		println!("{:?}", x);
	}

    println!("I'm called from C with a value of {:?}", unsafe {CStr::from_ptr(a).to_str().unwrap()} );
}

