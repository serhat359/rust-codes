use std::mem;

pub struct Stack<T> {
    list_head: Option<Box<Node<T>>>,
}

pub struct Node<T> {
    next: Option<Box<Node<T>>>,
    value: T,
}

impl<T> Stack<T> {
    pub fn is_empty(&self) -> bool {
        self.list_head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut node = &self.list_head;
        let mut i = 0;
        loop {
            match *node {
                Some(ref n) => {
                    i+=1;
                    node=&n.next;
                }
                None => {
                    return i;
                }
            }
        }
    }
    
    pub fn new() -> Stack<T> {
        Stack {list_head: None}
    }

    pub fn push(&mut self, elt: T) {
        self.push_front_node(Box::new(Node::new(elt)))
    }

    fn push_front_node(&mut self, mut new_head: Box<Node<T>>) {
        match self.list_head {
            None => {
                self.list_head = Some(new_head);
            }
            Some(ref mut head) => {
                mem::swap(head, &mut new_head);
                head.next = Some(new_head);
            }
        }
    }

    /// Provide a forward iterator
    #[inline]
    pub fn iter(&self) -> ListIterator<T> {
        ListIterator {nelem: self.len(), head: &self.list_head}
    }
}

impl<T> Node<T> {
    fn new(v: T) -> Node<T> {
        Node {value: v, next: None}
    }
}

pub struct ListIterator<'a, T: 'a> {
    head: &'a Option<Box<Node<T>>>,
    nelem: usize,
}

impl<'a, A> Iterator for ListIterator<'a, A> {
    type Item = &'a A;
    
    #[inline]
    fn next(&mut self) -> Option<&'a A> {
        if self.nelem == 0 {
            return None;
        }
        self.head.as_ref().map(|head| {
            self.nelem -= 1;
            self.head = &head.next;
            &head.value
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.nelem, Some(self.nelem))
    }
}

pub fn test() {
    let mut st = Stack::new();

    st.push(3);
    st.push(5);

    for x in 1..5000000 {
        st.push(x);
    }

    let mut x = st.list_head;

    while let Some(val) = x {
        let n = *val;
        //print!("{:?},", n.val);
        x = n.next;
    }

}