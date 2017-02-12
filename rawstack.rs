struct Stack {
    top: Option<Box<Node>>
}

struct Node {
	val: i32,
    next: Option<Box<Node>>
}

impl Stack {
    pub fn new() -> Stack{
    	Stack{
    		top: Option::None
    	}
    }

    pub fn push(&mut self, val: i32){
    	self.top = Some(Box::new(Node{ val: val, next: self.top.take() }));
    }

    pub fn peek(&self) -> i32{
    	match self.top {
    	    Some(ref boxed) => (*boxed).val,
    	    None => panic!("The stack is empty"),
    	}
    }

    pub fn pop(&mut self) -> i32{
    	let ptr: Box<Node> = if self.top.is_some() {
    		self.top.take().unwrap()
    	}
    	else {
    	    panic!("The stack is empty")
    	};

    	let node: Node = *ptr;

    	let retval = node.val;

    	self.top = node.next;

    	retval
    }
}

fn main() {
    let mut st = Stack::new();

    st.push(3);
    st.push(5);

    for x in 1..500000 {
        st.push(x);
    }

    println!("{}", st.pop());

    st.pop();

    println!("{}", st.peek());

    let mut x: Option<Box<Node>> = st.top;

    while x.is_some() {
    	let val: Box<Node> = x.expect("");
    	let n: Node = *val;
    	//print!("{:?},", n.val);
    	x = n.next;
    }
}