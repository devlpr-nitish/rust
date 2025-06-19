


fn main(){

    // variables are not mutable in rust
    // so we have to explicitly do using mut keyword
    let mut x = 12;
    x += 1;
    println!("{}", x);

    // constants should be in upper case always
    const  NUM:i32 = 132;
    // num += 3; // we can not do this
    println!("{}", NUM);


    // shadowing
    // we can define a variable of same name
    let mut y = 32;

    // here y is not &str(string slice) type 
    let y = "nitish";






    // Tuples

    let tup = (12, "Nitish");

    // way to access values in tuple
    let (val, name) = tup;
    let val = tup.0;
    let name = tup.1;



    // Arrays

    let arr = [12,13,42];
    
    arr[0];

    let arr = [0; 5];
    // println!("{}", arr[17]); // index out of bound



    

}