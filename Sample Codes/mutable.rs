fn main(){
	let mut s:String =String::from("Hello World");  // default is immutable if mutable then can change the value later 
	println!("{}",s); 				// but if you are using mut then need to change later otherwise generate error
	s=String::from("World");   			// also before changing you need atleast one read operation here println
	println!("{}",s);
	
	
	let mut a:String=String::from("Sample");
	let  b=mut_change(&mut a);
	println!("After function call b = {}",b);
}
fn mut_change(sl : &mut String)->&mut String
{
	sl.push_str(" Program");
	println!("Inside function After Push s= {} ",sl);
	sl
} 
