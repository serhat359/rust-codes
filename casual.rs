fn main(){
	let num = 6;

	let a = make_name(&num);

	println!("{:?}", a);

}

fn make_name(x: &i32) -> Name{
	let strct = Name { field: &x };

	return strct;
}

#[derive(Debug)]
struct Name<'a> {
    field: &'a i32
}
