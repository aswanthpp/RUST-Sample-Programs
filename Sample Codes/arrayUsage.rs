fn main() {
	let mut arr: [u8;4] = [1,2,3,4];		// array initilization 
	println!("{:?}",arr);
	for i in arr.iter() {			//array iteration 
		print!("{} ",i);
	}
	println!();
	arr[1]=10;   
	array_passing(arr);
	println!("{:?}",arr);
	
}
fn array_passing(a :  [u8]){			//array passing
	a[3]=44;
} 
