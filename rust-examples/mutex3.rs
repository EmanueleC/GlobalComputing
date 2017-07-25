use std::sync::Mutex;
use std::sync::Arc;
use std::thread;

fn main() {
	let counter = Arc::new(Mutex::new(0));
	let counter_clone1 = counter.clone();
	let handle = thread::spawn(move || {
		let mut num = counter_clone1.lock().unwrap();
		*num += 1;
	});

	let counter_clone2 = counter.clone();
	let handle2 = thread::spawn(move || {
		let mut num2 = counter_clone2.lock().unwrap();
		*num2 += 1;
	});
	handle.join().unwrap();
	handle2.join().unwrap();
	println!("{:?}", counter);
}
