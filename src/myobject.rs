use std::fmt::{ Display, Formatter, Result };
use std::cell::Cell;

pub struct MyObject {
    pub value: i32,
    pub text: String,
    pointer: usize,
    mutable_field: Cell<i32>
}

impl MyObject {
    pub fn new(name: &str, value: i32) -> MyObject{
    	MyObject{
    		text: name.to_string(),
    		value: value,
            pointer: 0,
            mutable_field: Cell::new(4)
    	}
    }

    pub fn test_method(&self) {
        println!("This is just a test");
        println!("{} {}", self.text, self.value);
    }
}

impl Display for MyObject {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{},{}", self.text, self.mutable_field.get())
    }
}
