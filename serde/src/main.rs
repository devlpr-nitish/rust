

use serde::{Serialize, Deserialize};
use serde_json::{self, Value};

#[derive(Serialize, Deserialize, Debug)]

struct User{
    username:String,
    password:String
}




fn main(){
    let u = User{
        username:String::from("Nitish"),
        password:String::from("3223")
    };
    
    // Serialize to JSON
    let json_str = serde_json::to_string(&u).unwrap();
    // match ser_str {
    //     Ok(str) => println!("{}", str),
    //     Err(_e) => println!("Error while converting to string")
    // }
    println!("{}", json_str);

    // Deserialize from JSON
    let des_str:User = serde_json::from_str(&json_str).unwrap();
    println!("{:?}", des_str);
}