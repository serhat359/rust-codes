

fn main(){
	let var = "deneme";

	let holder = StrHolder::new(var);

	println!("{}", holder.to_string());
}

struct StrHolder<'a> {
    field: &'a str
}

impl<'a> StrHolder<'a> {
    fn new(string: &str) -> StrHolder {
    	StrHolder{
    		field: string
    	}
    }

    fn to_string(&self) -> &'a str{
    	return self.field;
    }
}