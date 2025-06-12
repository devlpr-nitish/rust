
struct Rectangle{
    height:f32,
    width:f32,
}

impl Rectangle{

    fn area(&self)->f32{
        return self.width * self.height;
    }

    fn perimeter(&self)->f32{
        return 2.0*(self.width+self.height);
    }

    fn some_thing(){
        println!("Hello from static function");
    }
}


fn main(){

    let r = Rectangle{
        height:20.0,
        width:21.2
    };


    println!("perimeter is {}", r.perimeter());
    println!("Area is {}", r.area());
    Rectangle::some_thing();
}