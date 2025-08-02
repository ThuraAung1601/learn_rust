trait Shape {
    fn area(&self) -> f32;
    fn cloner(&self) -> Box<dyn Shape>;
}

struct ShapeVector(Vec<Box<dyn Shape>>);

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Box<dyn Shape> {
        self.cloner()
    }
}

impl Clone for ShapeVector {
    fn clone(&self) -> Self {
        let cloned_vector = self.0.iter().map(|s| s.clone()).collect();
        ShapeVector(cloned_vector)
    }
}

#[derive(Debug)]
struct Circle {
    x: f32,
    y: f32,
    radius: f32
}

impl Circle {
    fn new(x: f32, y: f32, r: f32) -> Box<dyn Shape> {
        Box::new(
            Circle {
                x: x,
                y: y,
                radius: r
            }
        )
    }
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        self.radius * self.radius * 3.1429
    }
    fn cloner(&self) -> Box<dyn Shape> {
        Box::new(
            Circle {
                x: self.x,
                y: self.y,
                radius: self.radius
            }
        )
    }
}

#[derive(Debug)]
struct Rectangle {
    x: f32,
    y: f32,
    w: f32,
    h: f32
}

impl Rectangle {
    fn new(x: f32, y: f32, w: f32, h: f32) -> Box<dyn Shape> {
        Box::new(
            Rectangle {
                x: x,
                y: y,
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
    fn cloner(&self) -> Box<dyn Shape> {
        Box::new(
            Rectangle {
                x: self.x,
                y: self.y,
                w: self.w,
                h: self.h
            }
        )
    }
}

fn main() {
    let c1 = Circle::new(1., 2., 3.75);
    let rec1 = Rectangle::new(1., 2., 3.23, 4.32);

    let v1 = vec![c1, rec1];
    // println!("{:?}", v1);

    let cloned = v1.clone();

    for shape in &cloned {
        println!("{}", shape.area());
    }
}
