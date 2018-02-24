fn main(){
	let f1_x=func();
	println!("Return Value = {}",f1_x);
}
fn func() -> &u8{			//scope of x is local to func   alert suer about scope during compile time 
	let x=4;
	&x
}
