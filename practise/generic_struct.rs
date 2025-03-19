// Create a struct Rectangle with the following properties:

// width (generic type)
// height (generic type)
// It should also have a method area() that calculates and returns 
// the area of the rectangle (width * height).
use std::ops::Mul;

#[derive(Debug)]
struct Rectangle <T> {
    w: T,
    h: T
}

impl <T:Mul<Output=T>+Copy>Rectangle<T>{
    fn new(w:T, h:T)->Self{
        Rectangle{w,h}
    }
    fn area(self) -> T{
        let area_here = self.w*self.h;
        area_here
    }
    fn double_size(&self) -> Rectangle<T> {
        
    }

}


fn main(){
    let rec = Rectangle::new(3,4);
    println!("{}",rec.area());
    let rec_f = Rectangle::new(4.5,3.2);
    println!("{}",rec_f.area());
}