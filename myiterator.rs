use std::slice::Iter;

#[derive(Debug)]
pub struct MyIterator<'a, T: Copy + 'a> {
    iter: Iter<'a, T>
}

impl<'a, T: Copy> MyIterator<'a, T> {
    pub fn new(arr: &[T]) -> MyIterator<T>{
    	MyIterator{
    		iter: arr.iter()
    	}
    }
}

impl<'a, T: Copy> Iterator for MyIterator<'a, T> {
    type Item = T;
	
	fn next(&mut self) -> Option<T> {
		let op: Option<&T> = self.iter.next();

		match op {
		    Some(expr) => Some(*expr),
		    None => None,
		}
	}
}