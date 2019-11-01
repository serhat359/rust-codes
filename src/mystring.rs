struct MyString {
	char_array: Vec<char>
}

impl MyString {
	fn from(string: &str) -> MyString {
		let mut chars = Vec::new();

		for c in string.chars() {
			chars.push(c);
		}

		MyString {
			char_array: chars
		}
	}
}

pub fn test() {
	let mut x = MyString::from("STRING");

	x.char_array.push(65 as char);

	for c in &x.char_array {
		print!("{},", *c);
	}

}