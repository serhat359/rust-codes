use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

fn main() {
	for i in ThreadIterator::new() {
		println!("{}", i);
	}
}

struct ThreadIterator {
	thread_count: i32,
	rx: Receiver<i32>
}

impl ThreadIterator {
	fn new() -> ThreadIterator{
		let (tx, rx) = mpsc::channel();

		let thread_count = 3;

		for i in 0..thread_count{
			let tx = tx.clone();

			thread::spawn(move || {
				//let answer = i;

				//thread::sleep(Duration::from_millis(1000 * i));

				let answer = fib(i + 40);

				tx.send(answer).ok().expect("Error while sending data");
			});
		}

		ThreadIterator{
			thread_count: thread_count as i32,
			rx: rx
		}
	}
}

impl Iterator for ThreadIterator {
	type Item = i32;
	
	fn next(&mut self) -> Option<i32> {
		if self.thread_count <= 0{
			return None;
		}
		else{
			self.thread_count -= 1;
			let answer = self.rx.recv().ok().expect("Error while receiving data");
			return Some(answer);
		}
	}
}

fn fib(x: i32) -> i32 {
	if x == 1 || x == 0 {
		x
	}
	else{
		fib(x-1) + fib(x-2)
	}
}

pub fn simple_thread(){
	let threads : Vec<_> = (0..10).map(|num|{ 
		thread::spawn(move ||{
			println!("Hello world! (from thread {})", num);
		})
	}).collect();

	for t in threads {
		t.join().unwrap();
	}
}
