extern crate libc;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::mem;
use std::ops::{Index, IndexMut};
use std::marker::PhantomData;


#[link(name = "wrapper")]
extern {
	fn create_array(array: *mut u32, num: u32); 
}

struct CArrayIter<'a, T> where T: 'a {
	curr: usize,
	data: &'a CArrayInt<'a, T>,
	phantom: PhantomData<&'a T>,
}

/* The iterator returns a non mutable reference from the buffer for each element. */
impl<'a, T> Iterator for CArrayIter<'a, T> {
	type Item = &'a T;

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

struct CArrayInt<'a, T> where T:'a + std::marker::Sized {
	ptr: *mut  T,
	size: usize,
	phantom: PhantomData<&'a T>,
}

impl<'a, T>  CArrayInt<'a, T> {
	fn new(size: usize) -> CArrayInt<'a, T> {
		
		let iptr: *mut u32 = unsafe { libc::malloc(size * mem::size_of::<T>() as libc::size_t) as *mut u32 };
		assert!(!iptr.is_null());

		unsafe {
			create_array(&mut *iptr , size as u32);
		}

		CArrayInt {
			ptr: iptr as *mut T,
			size: size,
			phantom: PhantomData,
		}
	}

	/* Create the iterator object */
	fn iter(&self) -> CArrayIter<T> {
		println!("GENERIC ITER");
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
impl<'a, T> IntoIterator for &'a CArrayInt<'a, T>  {
	type Item = &'a T;
	type IntoIter = CArrayIter<'a, T>;

	/* Define the into_iterator function that tells how the iterator of this struct is like */
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a, T> Index<u32> for CArrayInt<'a, T> {
	type Output = T;

	fn index(&self, index: u32) -> &Self::Output {
		unsafe {
			self.ptr.offset(index as isize).as_ref().unwrap()
		}
	}
}

/* Index operator that return mutable references, so that index operator can update the buffers */
impl<'a, T> IndexMut<u32> for CArrayInt<'a, T>  {
	fn index_mut(&mut self, index: u32) -> &mut T  {
		unsafe {
			self.ptr.offset(index as isize).as_mut().unwrap()
		}
	}
}

/* Destructor that frees up the memory given by libc */
impl<'a, T> Drop for CArrayInt<'a, T> {
	fn drop(&mut self) {
		println!("Dropping!");
		unsafe {
			libc::free(self.ptr as *mut libc::c_void);
		}
	}
}


#[no_mangle]
pub extern "C" fn callback(a: *const c_char) {

	let mut ptr : CArrayInt<u32> = CArrayInt::new(5);

	for i in 0..5 {
		println!("{:?}", ptr[i]);
		ptr[i] = ptr[i]*ptr[i];
	}

	for x in &ptr {
		println!("{:?}", x);
	}

    println!("I'm called from C with a value of {:?}", unsafe {CStr::from_ptr(a).to_str().unwrap()} );
}

