
extern crate futures;

use futures::Future;
use futures::Sink;
use futures::Stream;
use futures::sync::mpsc;
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;
use std::ops::{DerefMut, Deref};
use std::str;
use std::thread;

const BUFFER_SIZE: usize = 1;

fn main() {

	let (mut tx, rx) = futures::sync::mpsc::channel(BUFFER_SIZE);

	/* 
	 * TODO: 
	 * Multiple files simultaneously 
	 * Each send will have a thread_id and string.
	 * each receive will check the thread_id and call the correct handler that writes to appropriate stream.
	 */

	thread::spawn(move || {
			println!("--> START THREAD");

			let file = File::open("src/dummy");

			match file {
				Ok(mut handle) => {
					loop {
						let mut contents : Box<[u8]> = vec![0; 512].into_boxed_slice();
						let ret = handle.read(contents.deref_mut());
							match ret {
								Ok(_size) => {
									if _size <= 0 {
										println!("Data size is {}", _size);
										break;
									}
									let sstr = String::from(str::from_utf8(contents.deref()).unwrap());
									match tx.send(sstr).wait() {
										Ok(new_tx) => tx = new_tx,
										Err(_) => panic!("Dead!!"),
									}
								}
								Err(_) => println!("Couldn't read."),
							}
					}
				}
				Err(_) => println!("File not found."),
			}

			println!("<-- END THREAD");
	});

	let result = rx.fold(String::from(""), |acc, val| {
			//println!("{:?}", val);
			Ok(val)
		}).wait();

}
