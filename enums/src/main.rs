

// enum Message{
//     Quit,
//     Move{x : i32, y : i32},
//     Write(String),
//     ChangeColor(u8,u8,u8)
// }

// // can have impl of enum

// impl Message {
    
//     fn show(){
//         println!("hello");
//     }
// }


// fn show_message(msg: Message){

//     match msg{
//         Message::Quit => println!("Quit message"),
//         Message::Move { x, y } => println!("Move to ({}, {})", x, y),
//         Message::ChangeColor(r,g,b) => println!("Change color to {}, {}, {}", r,g,b),
//         Message::Write(text) => println!("{}", text)
//     }
// }

// fn main(){
//     let msg1 = Message::Quit;
//     let msg2 = Message::Move { x: 23, y: 312 };

    
//     show_message(msg1);
//     show_message(msg2);

    
//     Message::show();
// }


// Option ENUM

fn main(){

    // let x = 6;
    // // let y = Some(4);
    // let y = None;

    // println!("{}", x + y.unwrap_or(0));


    // match

    let s = Some(32);

    match s {
        Some(val)=>println!("{}",val),
        _ => println!("Nothing"),
    }


    if let Some(val) = s{
        println!("{:?}", val);
    }


}