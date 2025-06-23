

// #[derive(Debug, Clone)]
// struct User{
//     username:String,
//     email:String,
//     active: bool
// }


// fn  main() {
    
//     let user1 = User{
//         username:String::from("devlprnitish"),
//         email:String::from("dev@gmail.com"),
//         active:true
//     };

//     let user2 = build_user(String::from("dev@vfd.com"), String::from("devm"));

//     let user3 = User{
//         username:String::from("Nitish"),
//         ..user2.clone()
//     };
    
//     println!("{:?}", user1);
//     println!("{:?}", user2);
//     println!("{:?}", user3);
// }


// fn build_user(email:String, username:String) -> User{
//     User{
//         email,
//         username,
//         active:false
//     }
// }



#[derive(Debug)]
struct Rectangle{
    height:u32,
    width:u32
}

impl Rectangle{
    fn get_area(&self) -> u32{
        return  self.height * self.width;
    }

    fn can_hold(&self, other:&Rectangle) -> bool{
        self.height > other.height && self.width > other.width
    }
}

// can have multiple impl block
// associated function
impl Rectangle {

    fn square(size:u32) -> Rectangle{
        Rectangle { height: size, width: size }
    }

}


fn main(){

    let rect = Rectangle{
        height:32,
        width:34
    };

    println!("{:#?}", rect);

    // let area = get_area(&rect);
    // let area = rect.get_area();

    // println!("Area of rectange is {}", area);


    let rect1 = Rectangle{
        height:12,
        width:23
    };

    let rect2 = Rectangle{
        height:40,
        width:12
    };


    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect1: {}", rect.can_hold(&rect2));

    println!("{:?}", rect1);
    println!("{:?}", rect2);


    let rect_square = Rectangle::square(32);
    println!("{:?}", rect_square);
    
}

// fn get_area(shape : &Rectangle) -> u32{
//     return  shape.height * shape.width;
// }