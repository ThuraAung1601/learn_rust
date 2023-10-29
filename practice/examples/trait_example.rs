// traits
// OOP, polymorphism

trait Shape {
    fn area(&self) -> f32;
}

struct Circle {
    x: f32,
    y: f32,
    radius: f32
}

impl Circle {
    fn new(x: f32, y: f32, radius: f32) -> Circle {
        Circle {
            x: x, y: y, radius: radius
        }
    }
    // fn area(c: Circle) -> f32 {
    //     3.142 * c.radius * c.radius
    // }
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        3.142 * self.radius * self.radius
    }
}


struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32
}

impl Rectangle {
    fn new(x: f32, y: f32, w: f32, h: f32) -> Rectangle {
        Rectangle {
            x: x, y: y, width: w, height: h
        }
    }
    // fn area(rec: Rectangle) -> f32 {
    //     rec.width * rec.height
    // }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

fn main() {
    let c1 = Circle::new(1., 2., 34.56);
    let r1 = Rectangle::new(1., 2., 20., 34.);

    // let circle_area = Circle::area(c1);
    // let rec_area = Rectangle::area(r1);

    let circle_area = c1.area();
    let rec_area = r1.area();

    println!("{}, {}", circle_area, rec_area);
}