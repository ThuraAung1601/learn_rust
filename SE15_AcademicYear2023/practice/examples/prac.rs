use rand::Rng;

fn gen_rand<R: Rng>(rng: &mut R, n: usize) -> Vec<f32> {
    let mut v1 = Vec::new();
    for i in 0..n {
        let x = rng.gen_range(-1. ..=100.);
        v1.push(x);
    }
    v1
}

fn gen_pts<R: Rng>(rng: &mut R, n: usize) -> Vec<(f32, f32)> {
    let mut v1 = Vec::new();
    for i in 0..n {
        let x = rng.gen::<f32>();
        let y = rng.gen::<f32>();
        v1.push((x, y));
    }
    v1
}

fn check_even(v: &[f32]) -> Vec<f32> {
    v.iter().filter(|&&s| s%2. == 0.).cloned().collect()
}

fn add_one(v: &[f32]) -> Vec<f32> {
    v.iter().map(|s| s+1.).collect()
}

fn find_max(v: &[String]) -> usize {
    v.iter().map(|s| s.len()).max().unwrap_or(0)
}

fn vflip(img: &[String]) -> Vec<String> {
    let mut flipped = Vec::new();
    for i in img.iter().rev() {
        flipped.push(i.clone());
    }
    flipped
}

fn blend(img1: &[String], img2: &[String]) -> Vec<String> {
    let mut blended = Vec::new();
    for (i, j) in img1.iter().zip(img2.iter()) {
        blended.push(i.clone());
        blended.push(j.clone());
    }
    blended
}

fn main() {
    let mut rng = rand::thread_rng();
    let args: Vec<String> = std::env::args().collect();
    let n = args[1].parse::<usize>().unwrap_or(0);
    // println!("{:?}", gen_rand(&mut rng, n));

    // let generated = gen_rand(&mut rng, n);
    let generated = vec![2., 3., 4., 5.];
    // let generated = Vec::new();
    // println!("{:?}", check_even(&generated));
    // println!("{:?}", add_one(&generated));
    
    let rows = vec!["<===>", "<<<>>>", "!@!@!"];
    let pixels: Vec<String> = rows.iter().map(|s| s.to_string()).collect();
    // println!("{:?}", find_max(&pixels));

    let rows1 = vec!["<<<>>>", "<<<>>><===>", "!@!@!"];
    let pixels1: Vec<String> = rows1.iter().map(|s| s.to_string()).collect();
    // println!("{:?}", find_max(&pixels1));

    // println!("{:?}", vflip(&pixels));
    // println!("{:?}", blend(&pixels, &pixels1));

    // let gen_ls = gen_pts(&mut rng, n);
    let gen_ls = vec![(100., 200.), (1., 2.), (3., 4.)];
    println!("{:?}", gen_ls);
    let mut sorted = gen_ls.clone();
    sorted.sort_by(|(x0, y0), (x1, y1)| x0.partial_cmp(x1).unwrap().then(y0.partial_cmp(y1).unwrap()));
    println!("{:?}", sorted);
}

// trait 

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

struct Circle {
    r: f32
}

impl Circle {
    fn new(r: f32) -> Box<dyn Shape> {
        Box::new(
            Circle {
                r: r
            }
        )
    }
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        self.r * self.r * 3.142
    }
    fn cloner(&self) -> Box<dyn Shape> {
        Box::new(
            Circle {
                r: self.r
            }
        )
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
    fn cloner(&self) -> Box<dyn Shape> {
        Box::new(
            Rectangle {
                w: self.w,
                h: self.h
            }
        )
    }
}

fn main() {
    let c1 = Circle::new(3.5);
    let r1 = Rectangle::new(3.21, 3.72);
    let shapes = vec![c1, r1];

    for i in &shapes {
        println!("{}", i.area());
    }

    let cloned = shapes.clone();
    for i in &cloned {
        println!("{}", i.area());
    }
}

// enum

#[derive(Debug)]
#[derive(Clone)]
enum Shape {
    Circle {
        r: f32
    },
    Rectangle {
        w: f32,
        h: f32
    }
}

impl Shape {
    fn area(&self) -> f32 {
        match self {
            Shape::Circle{r} => r*r*3.142,
            Shape::Rectangle{w, h} => w*h,
            _ => panic!("Error")
        }
    }
    fn cloner(&self) -> Self {
        match self {
            Shape::Circle{r} => Shape::Circle{r: *r},
            Shape::Rectangle{w, h} => Shape::Rectangle{w: *w, h: *h},
            _ => panic!("Error")
        }
    }
}

fn main() {
    let c1 = Shape::Circle {
        r: 32.12
    };
    let r1 = Shape::Rectangle {
        w: 21.34, h: 231.3
    };
    let c2 = c1.cloner();
    let c3 = c1.clone();

    println!("{:?}", c1);
    println!("{:?}", c2);
    println!("{:?}", c3);

    println!("{:?}", c1.area());

    let v1 = vec![c1, r1.clone(), c2, c3, r1.cloner()];
    let mut v2 = v1.clone();
    println!("{:?}", v1);
    println!("{:?}", v2.pop()); // last indexed
}

// nibble swap

fn main() {
    let b = 0x32;
    let high_nibble = b >> 4;
    let low_nibble = b & 0xF;
    let swapped = low_nibble << 4 | high_nibble;
    println!("{:0x}", swapped);
}

// bubble sort

fn main() {
    let mut v1 = vec![1, 2, 3, 4, 100, 2, 1, 2];
    for i in 0..v1.len() {
        for j in 0..v1.len()-i-1 {
            if v1[j] < v1[j+1] {
                let temp = v1[j];
                v1[j] = v1[j+1];
                v1[j+1] = temp;
            }
        }
    }
    println!("{:?}", v1);
}