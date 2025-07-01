use std::collections::HashMap;

use unicode_segmentation::UnicodeSegmentation;

fn main(){

    // let a = [2,3,4,1];

    // let mut v : Vec<i32>= Vec::new();

    // let len = v.len();

    // v.push(31);
    // v.push(13);
    // v.push(31);
    // v.push(13);

    // println!("{}", v[0]);


    // let mut v2 = vec![3,4,2];

    // // let len2 = v2.len();
    // // let val = v2[0];

    // // println!("{}", val);
    // // println!("{}", v2[len2-1]);
    // // println!("{}", v2[0]);

    // // match v2.get(1) {
    // //     Some(val) => println!("Element {} is pesent ", val),
    // //     None => println!("Element is not present at index 29")
    // // }

    // for i in &mut v2{
    //     *i += 10;
    // }

    // for i in &v2{
    //     println!("{}", i);
    // }



    // enum SpreadSheet{
    //     Int(i32),
    //     Text(String),
    //     Float(f32)
    // }


    // let row = vec![
    //     SpreadSheet::Int(32),
    //     SpreadSheet::Text(String::from("Hello")),
    //     SpreadSheet::Float(4.32)
    // ];


    // match &row[0] {
    //     SpreadSheet::Int(i) => println!("It is an integer"),
    //     _ => println!("Not an integer")
    // }




    // String


    // let s = String::from("value");

    // for b in s.bytes(){
    //     println!("{} ", b);
    // }

    // for b in s.chars(){
    //     println!("{} ", b);
    // }


    // for g in s.graphemes( true){
    //     println!("{} ", g);
    // }


    // HashMap

    let blue = String::from("Blue");
    let red = String::from("red");

    let mut map = HashMap::new();

    map.insert(blue, 10);
    map.insert(red, 20);


    let team_name = String::from("Blue");

    let score = map.get(&team_name);

    for (key, val) in &map{
        println!("{} -> {}", key, val);
    }

    map.entry(team_name.clone()).or_insert(30);
    map.entry(team_name).or_insert(40);

    for (key, val) in &map{
        println!("{} -> {}", key, val);
    }


}