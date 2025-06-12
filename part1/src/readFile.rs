use std::fs;

fn main(){

    let content = fs::read_to_string("a.txt");

    match content{
        Ok(content) => println!("{}", content),
        Err(e) => println!("Error while reading file {} ", e)
    }
}