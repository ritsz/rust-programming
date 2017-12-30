extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use futures::Async;
use futures::Future;
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;
use std::str;
use tokio_core::reactor::Core;

#[derive(Debug)]
struct FileHandler {
	handle: File,
}

type Poll<T, E> = Result<Async<T>, E>;

impl Future for FileHandler {
	type Item = String;
	type Error = std::io::Error;

	fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
		let mut contents = String::new();
		self.handle.read_to_string(&mut contents);
		Ok(Async::Ready(contents))
	}

}

fn main() {
	let mut core = Core::new().unwrap();
	let handle = core.handle();

	let fd = FileHandler {
		handle: File::open("src/dummy").unwrap(),
	};

	let response = fd.then(move |data| {
							println!("{:?}", data);
							data
						});

	core.run(response);
	println!("TEST");

}
