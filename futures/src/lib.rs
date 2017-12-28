extern crate rand;
extern crate futures;

use std::thread;
use std::time::Duration;
use rand::distributions::{Range, IndependentSample};

// This function sleeps for a bit, then returns how long it slept.
pub fn sleep_a_little_bit() -> u64 {
	let mut generator = rand::thread_rng();
	let possibilities = Range::new(0, 2000);

	let choice = possibilities.ind_sample(&mut generator);

	let a_little_bit = Duration::from_millis(choice);
	thread::sleep(a_little_bit);
	choice
}


use futures::Future;
use futures::sync::oneshot;

pub fn one_shot() {
	let (tx, rx) = oneshot::channel();

	// We can spawn a thread to simulate an action that takes time, like a web
	// request. In this case it's just sleeping for a random time.
	thread::spawn(move || {
			println!("--> START");

			let waited_for = sleep_a_little_bit();
			println!("--- WAITED {}", waited_for);
			// This consumes the sender, we can't use it afterwards.
			tx.send(waited_for);

			println!("<-- END");
	});

	// Now we can wait for it to finish
	let result = rx.wait()
		.unwrap();
	println!("{:?}", result);
}

use futures::future::join_all;

const NUM_OF_TASKS: usize = 10;

pub fn join_all_channels() {
	let mut rx_set = Vec::new();

	// Next we'll spawn up a bunch of threads doing 'something' for a bit then sending a value.
	for index in 0..NUM_OF_TASKS {
		// Here we create a future, this is a `oneshot` value which is consumed after use.
		let (tx, rx) = futures::oneshot();
		// Add the reciever to the vector we created earlier so we can collect on it.
		rx_set.push(rx);

		// Spawning up a thread means things won't be executed sequentially, so this will actually
		// behave like an asynchronous value, so we can actually see how they work.
		thread::spawn(move || {
				println!("{} --> START", index);

				let waited_for = sleep_a_little_bit();
				println!("{} --- WAITED {}", index, waited_for);

				// Here we send back the index (and consume the sender).
				tx.send(index);

				println!("{} <-- END", index);
				});
	}

	// `join_all` lets us join together the set of futures.
	let result = join_all(rx_set)
		// Block until they all are resolved.
		.wait()
		// Check they all came out in the right order.
		.map(|values|
				values.iter()
				.enumerate()
				.all(|(index, &value)| index == value))
		// We'll be lazy and just unwrap the result.
		.unwrap();

	println!("Job is done. Waited on all channels together. Values returned in order: {}", result);
}

use futures::future::ok;
use futures::stream::Stream;
use futures::sync::mpsc;
use futures::Sink;

pub fn iterate_over_channels() {
	// We're using a bounded channel here with a limited size.
	let (mut tx, rx) = mpsc::channel(10);

	thread::spawn(move || {
			println!("--> START THREAD");
			// We'll have the stream produce a series of values.
			for _ in 0..10 {

				let waited_for = sleep_a_little_bit();
				println!("--- THREAD WAITED {}", waited_for);

				// When we `send()` a value it consumes the sender. Returning
				// a 'new' sender which we have to handle. In this case we just
				// re-assign.
				match tx.send(waited_for).wait() {
					// Why do we need to do this? This is how back pressure is implemented.
					// When the buffer is full `wait()` will block.
					Ok(new_tx) => tx = new_tx,
					Err(_) => panic!("Oh no!"),
				}
			}
			println!("<-- END THREAD");
			// Here the stream is dropped.
	});

	// We can `.fold()` like we would an iterator. In fact we can do many
	// things like we would an iterator.
	let sum = rx.fold(0, |acc, val| {
				// Notice when we run that this is happening after each item of
				// the stream resolves, like an iterator.
				println!("--- FOLDING {} INTO {}", val, acc);
				// `ok()` is a simple way to say "Yes this worked."
				// `err()` also exists.
				ok(acc + val)
		})
		.wait()
		.unwrap();
	println!("SUM {}", sum);
}


#[cfg(test)]
mod tests {
    #[test]
    fn one_shot_works() {
		use super::one_shot;
		one_shot();
    }

    #[test]
	fn join_all_works() {
		use super::join_all_channels;
		join_all_channels();
	}

	#[test]
	fn iterate_works() {
		use super::iterate_over_channels;
		iterate_over_channels()
	}
}
