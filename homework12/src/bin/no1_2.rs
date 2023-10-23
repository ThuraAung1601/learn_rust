trait Shape {
    fn rep_string(&self) -> String;
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
        let cloned_shapes: Vec<Box<dyn Shape>> = self.0.iter().map(|shape| shape.clone()).collect();
        ShapeVector(cloned_shapes)
    }
}

struct Circle {
    x: i32,
    y: i32,
    radius: f32,
}

impl Circle {
    fn new(x: i32, y: i32, radius: f32) -> Box<dyn Shape> {
        Box::new(Circle { x, y, radius })
    }
}

impl Shape for Circle {
    fn rep_string(&self) -> String {
        format!("<Circle: {}, {}, {}>", self.x, self.y, self.radius)
    }

    fn area(&self) -> f32 {
        3.14 * self.radius * self.radius
    }

    fn cloner(&self) -> Box<dyn Shape> {
        Box::new(Circle {
            x: self.x,
            y: self.y,
            radius: self.radius,
        })
    }
}

struct Rectangle {
    x: i32,
    y: i32,
    width: f32,
    height: f32,
}

#[allow(dead_code)]
impl Rectangle {
    fn new(x: i32, y: i32, width: f32, height: f32) -> Box<dyn Shape> {
        Box::new(Rectangle { x, y, width, height })
    }
}

impl Shape for Rectangle {
    fn rep_string(&self) -> String {
        format!("<Rectangle: {}, {}, {}, {}>", self.x, self.y, self.width, self.height)
    }

    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn cloner(&self) -> Box<dyn Shape> {
        Box::new(Rectangle {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        })
    }
}

#[derive(Clone)]
struct Triangle {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
}

#[allow(dead_code)]
impl Triangle {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> Box<dyn Shape> {
        Box::new(Triangle { x1, y1, x2, y2, x3, y3 })
    }
}

impl Shape for Triangle {
    fn rep_string(&self) -> String {
        format!("<Triangle: {}, {}, {}, {}, {}, {}>", self.x1, self.y1, self.x2, self.y2, self.x3, self.y3)
    }

    fn area(&self) -> f32 {
        let area = 0.5 * (((self.x1 - self.x3) * (self.y2 - self.y1) - (self.x1 - self.x2) * (self.y3 - self.y1)) as f32);
        area
    }

    fn cloner(&self) -> Box<dyn Shape> {
        Box::new(Triangle {
            x1: self.x1,
            y1: self.y1,
            x2: self.x2,
            y2: self.y2,
            x3: self.x3,
            y3: self.y3,
        })
    }
}

fn main() {
    let circle1 = Circle::new(20, 20, 30.0);
    println!("Area: {}", circle1.area());
    println!("Original: {}", circle1.rep_string());
    let circle2 = circle1.cloner();
    println!("The cloned one: {}", circle2.rep_string());

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { x: 20, y: 20, radius: 2.0 }),
        Box::new(Rectangle { x: 20, y: 20, width: 3.0, height: 4.0 }),
        Box::new(Triangle { x1: -2, y1: 1, x2: 3, y2: 2, x3: 1, y3: 5 })
    ];

    let mut cloned_shapes: Vec<Box<dyn Shape>> = Vec::new();
    for shape in &shapes {
        cloned_shapes.push(shape.cloner());
    }

    for shape in &cloned_shapes {
        println!("Cloned Area for {} is {}", shape.rep_string(), shape.area());
    }
}

#[allow(dead_code)]
fn input_shape_list() -> Vec<Box<dyn Shape>> {
    vec![
        Circle::new(0, 0, 1.0),
        Circle::new(50, 50, 15.0),
        Rectangle::new(40, 40, 20.0, 20.0),
        Rectangle::new(10, 40, 15.0, 10.0),
        Triangle::new(-2, 1, 3, 2, 1, 5),
    ]
}

#[allow(dead_code)]
const EXPECTED_001: &[&str] = &[
    "<Circle: 0, 0, 1>",
    "<Circle: 50, 50, 15>",
    "<Rectangle: 40, 40, 20, 20>",
    "<Rectangle: 10, 40, 15, 10>",
    "<Triangle: -2, 1, 3, 2, 1, 5>"
];

#[allow(dead_code)]
const EXPECTED_002: &[&str] = &[
    "<Circle: 0, 0, 1>, area: 3.14",
    "<Circle: 50, 50, 15>, area: 706.50",
    "<Rectangle: 40, 40, 20, 20>, area: 400.00",
    "<Rectangle: 10, 40, 15, 10>, area: 150.00",
    "<Triangle: -2, 1, 3, 2, 1, 5>, area: 8.50"
];

#[test]
fn test_shapes_001() {
    let shape_list = input_shape_list();
    let omap = shape_list.iter().map(|s| s.rep_string());
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_001);
}

#[test]
fn test_shapes_002() {
    let shape_list = input_shape_list();
    let omap = shape_list.iter().map(
    |s| format!("{}, area: {:.2}", s.rep_string(), s.area()) );
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_002);
}

#[test]
fn test_shapes_003() {
    let input_list = input_shape_list();
    let shape_list = input_list.clone();
    let omap = shape_list.iter().map(
    |s| format!("{}, area: {:.2}", s.rep_string(), s.area()) );
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_002);
}

#[test]
fn test_shapes_004() {
    let input_list = input_shape_list();
    let mut cloned_shapes: Vec<Box<dyn Shape>> = Vec::new();

    for shape in &input_list {
        cloned_shapes.push(shape.cloner());
    }

    let omap = cloned_shapes.iter().map(|s| format!("{}, area: {:.2}", s.rep_string(), s.area()));
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_002);
}