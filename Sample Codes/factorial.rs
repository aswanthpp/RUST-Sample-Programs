fn main(){
	let n : u64=5;
	let  ans : u64 =fact(n);
	println!("Ans : {}",ans);
	
	
	let arr:[u64;6];
	let mut x : u64;
	let i :u64;
	for i in 1..(arr.len()) {
		x=x*i;
		
	}
	println!("\n x is {}",x);
}
fn fact(num :  u64) ->  u64 {
	if num==1 || num ==0 {
		return num
	}
	
	return num*fact(num-1);
}
