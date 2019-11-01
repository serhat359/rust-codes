pub mod codes;
pub mod myiterator;
pub mod philosophers;
pub mod concurrency;
pub mod closure;
pub mod linked_list_found_online;
pub mod myobjecttest;
pub mod myobject;
pub mod mystring;
pub mod rawstack;
pub mod stack;
pub mod stack_found_online;
pub mod stack_with_enum;

macro_rules! dump {
    ($expression:expr) => (
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression);
    );
}

fn main(){
	//codes::test();

	//philosophers::test();

	let mut str: String = "some text".to_string();

	str.push_str("!!!");

	println!("{}", str);

	dump!(str);

	let asd = "";

	let _asd2 = asd.to_string();

	// Terminating the program, do not edit!!
	println!("\nThe program has terminated.");
}