use std::f32::consts::PI;


// trait

trait Shape{
    fn area(&self) -> f32;
}


// Rectangle
struct Rect {
    width: f32,
    height: f32,
}

impl Shape for Rect {
    fn area(&self) -> f32 {
        return self.height * self.width;
    }
}


// Circle
struct Circle {
    radius: f32,
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        return PI * self.radius * self.radius;
    }
}

// using Shape trait here

fn print_area_of_shape<T : Shape>(s: T) {
    println!("{}", s.area());
}

fn main(){

    let r = Rect{
        width: 12.0,
        height:10.0
    };

    let c = Circle{
        radius: 10.2
    };

    print_area_of_shape(r);
    print_area_of_shape(c);
}
