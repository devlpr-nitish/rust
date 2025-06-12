use std::f32::consts::PI;

enum Shape{
    Circle(f32),
    Rectangle(f32,f32)
}

impl Shape{

    fn area(&self) -> f32{
        return match self{
            Shape::Circle(r) => PI *  r * r,
            Shape::Rectangle(w,h)  => w*h
        };
    }
}


fn main(){

    let shape = Shape::Circle(10.5);

    println!("{}", shape.area());
}