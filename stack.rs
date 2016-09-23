use std::boxed::Box;
use std::ops::Deref;

struct Stack {
    top: Node
}

#[derive(Clone)]
enum Node {
	Val(i32, Box<Node>),
	Null
}

impl Stack {
    pub fn new() -> Stack{
    	Stack{
    		top: Node::Null
    	}
    }

    pub fn push(&mut self, value: i32){
    	let old_node: Node = self.top.clone();

    	let new_node: Node = Node::Val(value, Box::new(old_node));

    	*self = Stack{ top: new_node };
    }

    pub fn peek(&self) -> i32{
    	match self.top {
    	    Node::Val(int, _) => int,
    	    Node::Null => panic!("Stack is empty"),
    	}
    }

    pub fn pop(&mut self) -> i32{
    	let result: i32 = self.peek();

    	let new_node: Node = match self.top {
    	    Node::Val(_, ref boxed) => boxed.deref().clone(),
    	    Node::Null => panic!("Stack is empty"),
    	};

    	*self = Stack{ top: new_node };

    	result
    }
}

fn main() {
	let mut st = Stack::new();

	st.push(3);
	st.push(5);

	println!("{}", st.peek());

	st.pop();

	println!("{}", st.peek());
}