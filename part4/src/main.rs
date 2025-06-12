use std::fmt::Display;



#[derive(Debug)]
struct User{
    username:String,
    password:String,
    age:u32
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User age is {}", self.age)
    }
}


fn main() {
    let u = User{
        username:String::from("nitish"),
        password:String::from("nitish"),
        age:21
    };

    println!("{}", u); // display
    println!("{:?}", u); // debug
}
