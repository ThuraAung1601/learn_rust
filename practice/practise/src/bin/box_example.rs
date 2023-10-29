// Box dyn

trait Shape {
    fn area(&self) -> f32;
}

#[derive(Debug)]
struct Circle {
    x: f32,
    y: f32,
    radius: f32
}

impl Circle {
    fn new(x: f32, y: f32, radius: f32) -> Box<dyn Shape> {
        Box::new(Circle {
            x: x,
            y: y,
            radius: radius
        })
    }
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        3.142 * self.radius * self.radius
    }
}

struct Rectangle {
    w: f32,
    h: f32
}

impl Rectangle {
    fn new(w: f32, h: f32) -> Box<dyn Shape> {
        Box::new(
            Rectangle {
                w: w,
                h: h
            }
        )
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.w * self.h
    }
}

fn main() {
    // Vector in a Box
    let v1 = vec![1, 2, 3];
    let b1 = Box::new(v1);
    println!("{:?}", b1);

    // Struct in a Box
    let c1 = Circle {
        x: 2.,
        y: 1.,
        radius: 32.1
    };
    let b2 = Box::new(c1);
    println!("{:?}", b2);

    // Object Shape in a Box
    let circle1 = Circle::new(20., 20., 30.0);
    println!("Area: {}", circle1.area());
}