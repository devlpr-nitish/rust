


fn main() {
    
	let res =	other_function(34,132);
	println!("{}", res);

	let condition = false;

	let num = if condition {5} else {6};
	println!("{}", num)

}



fn other_function(mut x: i32, y: i32)-> i32{

	if x < y {
		x = y;		
	}else if x == y {
		return y;
	}
	
	// this is the way we return in rust
	x
}
