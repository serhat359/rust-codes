use std::boxed::Box;

enum Node {
	Cons(i32, Box<Node>),
    Nil,
}

impl Node {
    pub fn new() -> Node{
    	Node::Nil
    }

    #[warn(unused_must_use)]
    pub fn push(self, value: i32) -> Node{
    	Node::Cons(value, Box::new(self))
    }

    #[warn(unused_must_use)]
    pub fn peek(&self) -> i32{
    	match *self {
    	    Node::Cons(val,_) => val,
    	    Node::Nil => panic!("Stack is empty"),
    	}
    }
}

use self::Node::*;

pub fn test() {	
	let mut st = Node::new();

	st = st.push(3);
	st = st.push(5);

	let _ = st.peek();

	for x in 1..5000000 {
	    st = st.push(x);
	}

	let mut x = st;
	while let Cons(_, next_node) = x {
        //print!("{:?},", n.val);
        x = *next_node;
    }

}