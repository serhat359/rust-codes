mod myobject;

fn main(){
	let mut x = myobject::MyObject::new("noname", 2);

    x.text = "new text".to_string();

    x.value = 6;

    println!("{}", x);
    //println!("{:?}", x);

    x.test_method();

}

