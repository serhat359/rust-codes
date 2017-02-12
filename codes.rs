use std::io;
use std::fmt::Display;
mod myiterator;
use myiterator::MyIterator;

fn main() {
	
}

pub fn arr_is_equal<T: PartialEq>(arr1: &[T], arr2: &[T]) -> bool{
	let mut iter1 = arr1.iter();
	let mut iter2 = arr2.iter();

	loop {
		let val1 = iter1.next();
		let val2 = iter2.next();

		let has_val1 = val1.is_some();
		let has_val2 = val2.is_some();

		if !has_val1 && !has_val2 {
			return true;
		}

		if !has_val1 || !has_val2 {
			return false;
		}

		if val1.unwrap() != val2.unwrap() {
			return false;
		}
	}
}

pub fn inc(i: &mut i32){
	*i += 1;
}

pub fn each<F, T>(arr: &[T], f: F) where F: Fn(&T, u32){
	let mut i = 0;

	for e in arr {
		f(e,i);
		i += 1;
	}
}

pub fn strtest(){
	let a: String = strreturn();

	println!("{}", a);

	let b: &str = staticstrreturn();

	println!("{}", b);
}

fn strreturn() -> String{
	let resstr: String = "Xhertas".to_string();
	return resstr;
}

fn staticstrreturn() -> &'static str{
	return "Xhertas static";
}

pub fn test_array_iter(){
	let texts = ["sad","cvsxc","asdas"];

	for s in texts.iter(){
		printstr(s);
	}
}

pub fn printstr(x: &str){
	println!("{}", x);
}

pub fn borrow_check(){
	let mut x = 5;
	{
		let y = &mut x;
		*y += 1;
	}
	println!("{}", x);
}

pub fn print_int(x: i32){
	println!("{}", x);
}

pub fn match_and_iterator(){
	let mut range = 0..3;

	loop {
		match range.next() {
			Some(x) => {
				println!("{}", x);
			},
			None => { break }
		}
	}
}

pub fn switch_test() {
	let program = "+ + * - /";
	let mut accumulator = 0;

	for token in program.chars() {
		match token {
			'+' => accumulator += 1,
			'-' => accumulator -= 1,
			'*' => accumulator *= 2,
			'/' => accumulator /= 2,
			_ => { } // ignore everything else
		}
	}

	println!("The program \"{}\" calculates the value {}", program, accumulator);
}

pub fn gcd(x: i32, y: i32) -> i32{
	let mut big = max(x,y);
	let mut small = min(x,y);

	while small != 0 {
		let temp = big % small;
		big = small;
		small = temp;
	}

	return big;
}

pub fn max(x: i32, y: i32) -> i32{
	return if x > y { x } else { y };
}

pub fn min(x: i32, y: i32) -> i32{
	return if x > y { y } else { x };
}

pub fn closure(){
	let printint = |x: i32|{ 
		println!("{}", x); 
	};

	printint(45);
}

pub fn string_test(){
	let example_string = "Hello World!";

	let strref = &example_string;
	let strclone = strref.clone();

	println!("{}", example_string);
	println!("{}", strref);
	println!("{}", strclone);

	let chars = strref.chars();

	for ch in chars {
		print!("{} ", ch);
	}

	let mutstr = "Hi";
	
	let printstr = |x: &str|{ print!("\n{}", x); };

	printstr(mutstr);

	// know the difference between String and &str
	let str1: &str = "world!";
	let str2: String = " third".to_string();

	let strconcat = str2.to_string() + &str1; // the first one needs to be String because %str's value vannot be changed
	let strconcat2 = format!("{}{}", &str1, &str2); // a more efficient way of doing it

	printstr(&strconcat);
	printstr(&strconcat2);
}

pub fn ok_expect(){
	let mut string = String::new();

	let result = io::stdin().read_line(&mut string);

	result
	.ok()
	.expect("Don't panic! I can handle this!");
}

// consumer example, like sum
pub fn linq_any(){
	let greater_than_forty_two = (0..100).find(|x| *x > 42);

	match greater_than_forty_two {
		Some(_) => println!("We got some numbers!"),
		None => println!("No numbers found :("),
	}
}

pub fn linq_fold(){
	let _sum = (1..4).fold(0, |sum, x| sum + x);
}

pub fn linq_collect(){
	let _ = (1..1000)
	.filter(|&x| x % 2 == 0)
	.filter(|&x| x % 3 == 0)
	.take(5)
	.collect::<Vec<i32>>();
}

pub fn mut_array(){
	let mut arr = [1,2,3];

	for i in &mut arr{
		*i += 2;
	}

	for i in &arr{
		println!("{}", i);
	}
}

pub fn function<T: Display>(i: T){
	print!("{}", i);
}

pub fn print_arr<T: Display>(arr: &[T]){
	let mut arr = arr.into_iter();

	let first = arr.next();

	if first.is_some() {
		print!("{}", first.unwrap());

		loop {
			match arr.next() {
				Some(val) => print!(",{}", val),
				None => break,
			}
		}

		println!("");
	}
}

pub enum MyResult<T, E> {
	Ok(T),
	Err(E),
}

pub enum MyOption<T> {
	None,
	Some(T),
}

pub fn result_ok_expect(){
	let string = "123";

	let parse_result: Result<u32, _> = string.parse::<u32>();

	let result_ok: Option<u32> = parse_result.ok();

	let result_value: u32 = result_ok.expect("Error while parsing the string");

	println!("{}", result_value);
}

impl<T> MyOption<T> {
	pub fn my_expect(self, err_text: &str) -> T {
		match self {
			MyOption::Some(expr) => expr,
			MyOption::None => panic!("{}", err_text),
		}
	}
}

pub fn arr_as_iter<A: Copy>(arr: &[A]) -> MyIterator<A> {
	myiterator::MyIterator::new(arr)
}