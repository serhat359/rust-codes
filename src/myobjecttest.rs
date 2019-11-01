use myobject::MyObject;

pub fn test(){
	let mut x = MyObject::new("noname", 2);

    x.text = "new text".to_string();

    x.value = 6;

    println!("{}", x);
    //println!("{:?}", x);

    x.test_method();

}

