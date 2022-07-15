use std::ops::Mul;

fn main(){
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let triangle = Triangle  { length: 3.0, height: 4.0 };

    println!("rectangle {}", rectangle.area());
    println!("triangle {}", triangle.area());
}

trait HasArea {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }

#[derive(Debug)]
struct Triangle  { length: f64, height: f64 }

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

impl HasArea for Triangle {
    fn area(&self) -> f64 { self.length * self.height / 2.0 }
}

fn area<T: HasArea>(t: &T) -> f64 { t.area() }