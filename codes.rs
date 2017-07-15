use std::io;
use std::fmt::Display;
use myiterator::MyIterator;

#[derive(Debug)]
pub struct Pair (Box<i32>, Box<i32>);

pub fn test() {

	let a = 3;

	println!("{}", a);

	println!("{}", a);
	


	
}

// 2.3 s on 10000000 loop
pub fn return_string() -> String{
	return "aldkjaslkdjlajdlkasjldajklshkjasdjkasdkjagsdhjagsdjhgahdgjasd".to_string();
}

// 2.2 s on 10000000 loop
pub fn create_string(){
	let _ = "aldkjaslkdjlajdlkasjldajklshkjasdjkasdkjagsdhjagsdjhgahdgjasd".to_string();
}

// 0.55 s on 10000000 loop
pub fn return_str() -> &'static str{
	return "aldkjaslkdjlajdlkasjldajklshkjasdjkasdkjagsdhjagsdjhgahdgjasd";
}

// 0.9 s on 10000000 loop
pub fn return_strptr(x: &str) -> &str{
	return &x;
}

// 0.6 s on 10000000 loop
pub fn return_strstr(x: String) -> String{
	return x;
}

// 10.5 s on 10000000 loop
pub fn return_strcreate(x: String) -> String{
	return x.to_string();
}

pub fn test_array_mut(){
	let mut x = [5,8,2];

	let printandinc = |x: &mut i32| {
		println!("{}", x);
		*x += 1;
	};

	eachmut(&mut x, |e,_| printandinc(e));

	print_arr(&x);
}

pub fn box_unwrap(){
	let adder_in_box: Box<Fn() -> i32> = make_adder(3,5);

	let adder = &adder_in_box;

	println!("{}", adder());
}

pub fn make_adder(a: i32, b: i32) -> Box<Fn() -> i32> {
	// return closure bu sekilde yapiliyor
	Box::new(move || a + b)
}

pub fn print_arr<T: Display>(arr: &[T]){
	let mut iter = arr.iter();

	print!("{{");

	if let Some(val) = iter.next() {
		print!("{}", val);
	}

	while let Some(val) = iter.next() {
		print!(",{}", val);
	}

	print!("}}");
}

pub fn arr_iter_into_iter(){
	let vec1 = vec![1, 2, 3];

	// `iter()` for vecs yields `&i32`. Destructure to `i32`.
	println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
	// `into_iter()` for vecs yields `i32`. No destructuring required.
	println!("2 in vec2: {}", vec1.into_iter().any(| x| x == 2));

	let array1 = [1, 2, 3];

	// `iter()` for arrays yields `&i32`.
	println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
	// `into_iter()` for arrays unusually yields `&i32`.
	println!("2 in array2: {}", array1.into_iter().any(|&x| x == 2));
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

pub fn each<F, T>(arr: &[T], f: F) where F: Fn(&T, u32){
	let mut i = 0;

	for e in arr {
		f(e,i);
		i += 1;
	}
}

pub fn eachmut<F, T>(arr: &mut [T], mut f: F) where F: FnMut(&mut T, u32){
	let mut i = 0;

	for e in arr {
		f(e,i);
		i += 1;
	}
}

pub fn test_array_iter(){
	let texts = ["sad","cvsxc","asdas"];

	for s in texts.iter(){
		println!("{}", s);
	}
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
			Some(ref x) => {
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

	big
}

pub fn max(x: i32, y: i32) -> i32{
	if x > y { x } else { y }
}

pub fn min(x: i32, y: i32) -> i32{
	if x > y { y } else { x }
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
	let _: Vec<i32> = (1..1000)
	.filter(|&x| x % 2 == 0)
	.filter(|&x| x % 3 == 0)
	.take(5)
	.collect::<Vec<i32>>();

	// vector olusturma boyle
	let _: Vec<i32> = vec![1, 2, 3];
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
	MyIterator::new(arr)
}