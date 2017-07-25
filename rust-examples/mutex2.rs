use std::sync::Mutex;
use std::thread;

fn main() {
	let counter = Mutex::new(0);
	let handle = thread::spawn(move || {
		let mut num = counter.lock().unwrap();
		*num += 1;
	});

	let handle2 = thread::spawn(move || {
		let mut num2 = counter.lock().unwrap();
		*num2 += 1;
	});
}
