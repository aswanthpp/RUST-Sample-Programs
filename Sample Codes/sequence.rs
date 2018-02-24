fn main(){
	//array slicing
	let num_arr: [u9;4]=[1,2,3,4];
	let two_num : &[u8] = &num_arr[0..2];		//slicing first three elemnets
	let all_number : &[u8] =&num_arr[..];		// all slice 
	
	
							// alle these slicing is immutable in nature (like view)
	
	//tuple
	
	
	//vectors 
	let mut vector1 : Vec<u8> = Vec::new();
	vector1.push(1);
	
	//hashmap
	
		
	//string slice
	let s="This is sample string";
	const s2: &'static str="This is 
	
}
