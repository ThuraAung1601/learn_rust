use rand::Rng;

#[derive(Debug)]
struct Circle {
    x: f32,
    y: f32,
    radius: f32,
}

#[derive(Debug)]
struct Layer {
    name: String,
    color: String,
    circles: Vec<Circle>,
}

// No 1.2
fn cal_average_area(layers: Vec<Layer>) -> Vec<(String, f32)> {
    let mut result = Vec::new();
    const PI: f32 = 3.142;
    for l in layers {
        let mut sum = 0.;
        for c in &l.circles {
            // println!("{:?}", c.radius);
            let area = PI * c.radius * c.radius;
            // println!("{}", area);
            sum += area;
        }
        let avg_area = sum/l.circles.len() as f32;
        result.push((l.name, avg_area))
    }
    return result;
}

// No 1.1
fn gen_layer(name: String, color: String, rng: &mut impl Rng) -> Layer {
    let num = rng.gen_range(20..=50); 
    let mut circles = Vec::new();
    for i in 0..num {
        let x = rng.gen_range(-100. ..= 100.);
        let y = rng.gen_range(-100. ..= 100.);
        let r = rng.gen_range(-10. ..= 20.);
        circles.push(Circle{x: x, y: y, radius: r});
    }
    Layer{
        name,
        color,
        circles,
    }
}

// No 1.1
fn gen_obj_layer_list(rng: &mut impl Rng, n: usize) -> Vec<Layer> {
    let mut layers = Vec::new();
    for i in 0..n {
        let name = format!("Layer {}", i+1);
        // let color = format!("#RRGGBBA");
        // println!("{:?}", gen_layer(name, color, rng));
        let r = rng.gen::<u8>();
        let g = rng.gen::<u8>();
        let b = rng.gen::<u8>();
        let a = rng.gen::<u8>();
        let color = format!("#{:02X}{:02X}{:02X}{:02X}",r, g, b, a);
        layers.push(gen_layer(name, color, rng));
    }
    return layers;
}

fn main() {
    let mut rng = rand::thread_rng();
    let n = 5;
    // println!("{:?}",gen_obj_layer_list(&mut rng, n));
    let result = cal_average_area(gen_obj_layer_list(&mut rng, n));
    // println!("{:?}", result);
}

#[test]
fn test_gen_obj_layer_list() {
    let mut rng = rand::thread_rng();
    let n = 5;
    let layers = gen_obj_layer_list(&mut rng, n);

    assert_eq!(layers.len(), n);

    for layer in &layers {
            assert!(!layer.circles.is_empty());
    }
}

#[test]
fn test_cal_average_area() {
    let layer = vec![Layer { 
        name: "Layer 1".to_string(),
        color: "#47C6E92B".to_string(), 
        circles: vec![
                    Circle { 
                            x: -47.73065, 
                            y: 32.04605, 
                            radius: 15.582638 },
                    Circle { 
                            x: 51.935318, 
                            y: -24.768623, 
                            radius: -7.6575804 } 
                ]
    }];
    let result = cal_average_area(layer);
    assert_eq!(vec![("Layer 1".to_string(), 473.58917)], result);
    assert_eq!(result[0].1, 473.58917);
}

