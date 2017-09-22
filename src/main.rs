pub mod codes;
pub mod myiterator;
pub mod philosophers;
pub mod concurrency;
pub mod closure;

fn main(){

	//codes::test();

	//philosophers::test();

	closure::test();

	let mut str: String = "some text".to_string();

	str.push_str("!!!");

	println!("{}", str);

	let _ = str.capacity();



	// Terminating the program, do not edit!!
	println!("\nThe program has terminated.");
}

