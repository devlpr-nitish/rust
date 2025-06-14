
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]

struct User{
    username:String,
    password:String
}

fn main(){

    let u = User{
        username:String::from("Nitish"),
        password:String::from("32121")
    };

    let mut v:Vec<u8> = Vec::new();

    let ans = u.serialize(&mut v);

    match ans{
        Ok(_) => println!("{:?}", v),
        Err(_) => println!("Error while serializing")
    }

    let user = User::try_from_slice(&v).unwrap();
    println!("{:?}", user.username);



}