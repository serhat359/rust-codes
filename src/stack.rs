use std::boxed::Box;
use std::ops::Deref;

#[deprecated(note="The stack implemented here uses self defined enum Node, which is not optimized by the compiler.
The code implemented in rawstack.rs is a lot more faster than this code, please use that. The use of Enum complicates stuff, do not use Enums for data holding")]
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

pub fn test() {
	let mut st = Stack::new();

    st.push(3);
    st.push(5);
    st.pop();
    st.push(5);

    let _ = st.peek();

    for x in 1..5000000 {
        st.push(x);
    }

    let mut x: Node = st.top;
    while let Node::Val(i,ptr) = x {
        let n: Node = *ptr;
        let _ = i;
        //print!("{:?},", i);
        x = n;
    }
}