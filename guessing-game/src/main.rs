use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;


fn main() {
    println!("Guess a number!");

    let rand_num = rand::thread_rng().gen_range(1, 101); 

    loop {
        println!("{}", "Enter your number..".yellow());

        let mut num = String::new();

        io::stdin().read_line(&mut num).expect("Error while taking input");

        // converting string to number
        let num:i32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match num.cmp(&rand_num) {
            Ordering::Greater => println!("{}", "you guess the greater number".red()),
            Ordering::Less => println!("{}", "you guess the lesser number".red()),
            Ordering::Equal => {
                println!("{}", "you won the game".green());
                break;
            }
        }
    }

}
