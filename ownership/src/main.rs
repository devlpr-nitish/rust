


// fn main(){

//     let x = 1;
//     let y = x; // Copy

//     println!("{}", y);
//     println!("{}", x);

//     let s1 = String::from("nitish");
//     // let s2 = s1; // Move
//     let s2 = s1.clone();

//     println!("{}", s1);
// }




// ownership in functions

// fn main(){
//     let s = String::from("nitish");

//     diff_fn(s);

//     println!("{}", s); // we can use s here

// }


// fn diff_fn(s: String) {
//     println!("{}", s);
// }


// Referance in funtions

// fn main(){

//     let s = String::from("Nitish");

//     // let (l, s1) = cal_len(s);
    
//     // println!("{}", s); // we can not use s here bcz it is moved to the function
//     // so use references to overcome this problem called Borrowing

//     let (l, s1) = cal_len(&s);
//     println!("{}", s);
//     println!("{}", l);
//     println!("{}", s1);

// }


// fn cal_len(s:&String) -> (usize, &String){
//     let l = s.len();

//     return (l, s);
// }


// fn main(){

//     let mut s = String::from("Nitish");

//     app(&mut s);

//     println!("{}", s);

// }

// fn app(s: &mut String){
//     s.push_str(" Kumar");
// }



// fn main(){

//     let mut s = String::from("Niitsh");

//     // let t = &mut s;
//     // // we can not have more than one mutable referances at a time
//     // let r = &mut s;

//     // But we can have more than one immutable referance at a time
//     let t = &s;
//     let r = &s;

//     // but if that mutable variable is passed as immutable ref. then you can not take it as a mutable ref.
//     // let k = &mut s;


//     // Overall we can not do read and write at a same time

//     // but after using that immutable ref. then we can take that as a mutable ref.

//     println!("{}, {}", r, t);

//     let k = &mut s;// this is valid not bcz we have used r and t


// }



// Slice

fn main(){

    let s = String::from("Nitish Kumar");

    let nitish = &s[0..6];
    let kumar = &s[7..];

    let total = &s[..];

    println!("{}", nitish);
    println!("{}", kumar);
    println!("{}", total);

    // same on array also

    let arr = [2,4,2,23,5];

    let slice = &arr[..];

    println!("{:?}", slice);
}