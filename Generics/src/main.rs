// use dotenv::dotenv;
// use std::env;

// traits
use std::{fmt::Display, ops::Add, ops::Mul};
#[derive(Copy, Clone)]

struct Rect<T> {
    width: T,
    height: T,
}

impl<T: Mul<Output = T> + Copy> Rect<T> {
    fn area(&self) -> T {
        return self.height * self.width;
    }
}

fn main() {
    // Evn variables 👀

    // dotenv().ok();
    // let var = env::var("REDIS_ADDRESS");

    // match var {
    //     Ok(str) => println!("{}", str),
    //     Err(_e) => println!("Error while reading env variable"),
    // }

    // Generics and Traits 👀

    println!("{}", sum(2, 4));

    display_numbers(2, 4);
    display_numbers("Nitish", "Kushwaha");
}

fn sum<T: Add<Output = T>>(a: T, b: T) -> T {
    return a + b;
}

fn display_numbers<T: Display>(a: T, b: T) {
    println!("{} {}", a, b);
}
