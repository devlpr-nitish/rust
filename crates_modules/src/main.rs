


mod utils;
mod math;


mod parent{
    pub fn show(){
        println!("parent show");
    }

    pub mod child {
        pub fn display(){
            super::show();
        }
    }
}

fn main(){
    crate::utils::greet();

    crate::math::basic::demo();

    parent::child:: display();
}