// Lab 12
// 3.2
trait Shape {
    fn rep_string(&self) -> String;
    // 3.3
    fn area(&self) -> f32;
    // 3.4
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
    // 3.2
    fn rep_string(&self) -> String {
        format!("<Circle: {}, {}, {}>", self.x, self.y, self.radius)
    }

    // 3.3
    fn area(&self) -> f32 {
        3.14 * self.radius * self.radius
    }

    // 3.4
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

fn main() {
    let circle1 = Circle::new(20, 20, 30.0);
    println!("Area: {}", circle1.area());
    println!("Original: {}", circle1.rep_string());
    let circle2 = circle1.cloner();
    println!("The cloned one: {}", circle2.rep_string());

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { x: 20, y: 20, radius: 2.0 }),
        Box::new(Rectangle { x: 20, y: 20, width: 3.0, height: 4.0 }),
    ];

    let mut cloned_shapes: Vec<Box<dyn Shape>> = Vec::new();
    for shape in &shapes {
        cloned_shapes.push(shape.cloner());
    }

    for shape in &cloned_shapes {
        println!("Cloned Area for {} is {}", shape.rep_string(), shape.area());
    }
}

fn input_shape_list() -> Vec<Box<dyn Shape>> {
    vec![
        Circle::new(0, 0, 1.0),
        Circle::new(50, 50, 15.0),
        Rectangle::new(40, 40, 20.0, 20.0),
        Rectangle::new(10, 40, 15.0, 10.0),
    ]
}

const EXPECTED_001: &[&str] = &[
    "<Circle: 0, 0, 1>",
    "<Circle: 50, 50, 15>",
    "<Rectangle: 40, 40, 20, 20>",
    "<Rectangle: 10, 40, 15, 10>",
];

const EXPECTED_002: &[&str] = &[
    "<Circle: 0, 0, 1>, area: 3.14",
    "<Circle: 50, 50, 15>, area: 706.50",
    "<Rectangle: 40, 40, 20, 20>, area: 400.00",
    "<Rectangle: 10, 40, 15, 10>, area: 150.00"
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
