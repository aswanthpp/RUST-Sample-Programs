fn main(){
	let s: String=String::from("hello");
	let a=5;
	println!("Inside main before call s = {} ",s);
	function(s); // string type ownership is transferred to sl hence after function call 
	//println!("Inside main after call s= {}",s);  // s will not print anything instead generate error
	
	println!("Inside main before call a = {}",a);  // for integer ownership is not applicabe
	function2(a);
	println!("Inside main after call a = {}",a);  // for integer ownership is not applicabe
	
}
fn function2(sl : i32){
	println!("Inside Function  sl = {}",sl);
}
fn function(sl: String){
	println!("Inside function  sl = {}",sl);   // string sl have ownership of string "hello" // see the diagram
}
