

fn main(){
    let s = String::from("NitishKaBabu");

    match find_first_a(s){
        Some(p) => println!("a found at index {}", p),
        None => println!("a not found in given string")
    }
}

fn find_first_a(str: String) -> Option<usize>{

    if let Some(pos) = str.find('a'){
        return Some(pos as usize)
    }

    return None
}