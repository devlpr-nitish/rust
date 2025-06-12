
#[derive(Debug, Copy, Clone)]
struct User{
    is_male:bool,
    age:u32
}


fn main(){
    let u1 = User{
        is_male:true,
        age:21
    };

    let u2 = u1;

    println!("{:?}, {:?}", u1, u1);
}