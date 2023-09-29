use rand::Rng;
use std::io;
use std::io::Write;

#[derive(PartialEq)]
#[derive(Clone)]
// enable the availability to copy the Point object
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: f32,
}


#[derive(Debug)]
struct RandConfig {
    x_min: f32, x_max: f32,
    y_min: f32, y_max: f32,
}

#[derive(Debug)]
#[derive(PartialEq)]
enum PointLocation {
    Inside(Point, f32),
    Outside(Point, f32),
}

fn gen_point_list(rng: &mut impl Rng, config: &RandConfig, num: usize) -> Vec<Point> {
    let mut pt_list: Vec<Point> = Vec::new();
    for _i in 0..num {
        let x = rng.gen_range(config.x_min ..= config.x_max);
        let y = rng.gen_range(config.y_min ..= config.y_max);
        // println!("{} {}", x, y);
        pt_list.push(Point{x, y});
    }
    return pt_list;
}

fn locate_n_point(circle: &Circle, pt_ls: &[Point]) -> Vec<PointLocation> {
    let mut loc_list = Vec::new();
    for pt in pt_ls {
        let dx = pt.x - circle.center.x;
        let dy = pt.y - circle.center.y;
        // println!("{} {}", dx, dy);
        let distance = (dx * dx + dy * dy).sqrt();

        if distance <= circle.radius {
            // pt.clone() in the PointLocation enum variants is to avoid ownership issues 
            // and ensure that each PointLocation variant contains its own copy of the Point.
            loc_list.push(PointLocation::Inside(pt.clone(), distance));
        } else {
            loc_list.push(PointLocation::Outside(pt.clone(), distance));
        }
    }
    return loc_list;
}

fn generate_svg_for_layer(layer: &Layer) -> String {
    let mut document = "<svg height=\"500\" width=\"500\" xmlns=\"http://www.w3.org/2000/svg\">\n".to_string();
    document.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#EEEEEE\" />\n");
    
    for (point, location) in layer.points.iter().zip(layer.points_location.iter()) {
        let (fill_color, shape) = match location {
            PointLocation::Inside(_) => (&layer.color_inside, "circle"),
            PointLocation::Outside(_) => (&layer.color_outside, "cross"),
        };
        
        match shape {
            "circle" => {
                let circle = format!(
                    "<circle cx=\"{}\" cy=\"{}\" fill=\"{}\" r=\"5\"/>\n",
                    point.x,
                    point.y,
                    fill_color
                );
                document.push_str(&circle);
            }
            "cross" => {
                let cross = format!(
                    "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" />\n",
                    point.x - 5.0,
                    point.y - 5.0,
                    point.x + 5.0,
                    point.y + 5.0,
                    fill_color
                );
                document.push_str(&cross);
                let cross = format!(
                    "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" />\n",
                    point.x - 5.0,
                    point.y + 5.0,
                    point.x + 5.0,
                    point.y - 5.0,
                    fill_color
                );
                document.push_str(&cross);
            }
            _ => (),
        };
    }
    
    document.push_str("</svg>");
    document
}

fn main() {
    // // No 3.1
    // let mut rng = rand::thread_rng();
    // let cfg = RandConfig{
    //     x_min: -1.5, x_max: 1.5,
    //     y_min: -1.5, y_max: 1.5
    // };
    // let c = Circle{
    //             center: Point{
    //                         x: -0.1, 
    //                         y: -0.1
    //                         }, 
    //             radius: 0.8
    // };
    // let pt_list = gen_point_list(&mut rng, &cfg, 5);
    // // println!("{:?}", pt_list);

    // // No 3.2
    // let loc_list = locate_n_point(&c, &pt_list);
    // for loc in loc_list {
    //    println!("{loc:?}");
    // }

    // Alternative No 3.3
    // // minimum value input
    // print!("Enter minimum value for x and y: ");
    // io::stdout().flush().unwrap();
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();
    // let min = input.trim().parse::<f32>().unwrap_or(0.0);
    // // println!("{}", x_min);

    // // maximum value input
    // print!("Enter maximum value for x and y: ");
    // io::stdout().flush().unwrap();
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();
    // let max = input.trim().parse::<f32>().unwrap_or(0.0);

    // // number of points value input
    // print!("Enter number of points: ");
    // io::stdout().flush().unwrap();
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();
    // let num = input.trim().parse::<f32>().unwrap_or(0.0);

    // // circle information
    // print!("Enter circle center x y and r:");
    // let mut input = String::new();
    // io::stdout().flush().unwrap();
    // io::stdin().read_line(&mut input).unwrap();
    // let mut parts = input.trim().split_whitespace();
    // let x = parts.next().unwrap().parse::<f32>().unwrap_or(0.0);
    // let y = parts.next().unwrap().parse::<f32>().unwrap_or(0.0);
    // let r = parts.next().unwrap().parse::<f32>().unwrap_or(0.0);
    // // println!("{} {} {}", x, y, r);

    let args: Vec<String> = env::args().collect();
    
    if args.len() != 6 {
        eprintln!("Usage: {} <x_min> <x_max> <y_min> <y_max> <radius>", args[0]);
        std::process::exit(1);
    }
    
    let x_min = f32::from_str(&args[1]).expect("Invalid x_min");
    let x_max = f32::from_str(&args[2]).expect("Invalid x_max");
    let y_min = f32::from_str(&args[3]).expect("Invalid y_min");
    let y_max = f32::from_str(&args[4]).expect("Invalid y_max");
    let radius = f32::from_str(&args[5]).expect("Invalid radius");
    
    let config = RandConfig {
        x_min,
        x_max,
        y_min,
        y_max,
    };
    
    let circle = Circle {
        center: Point {
            x: 0.0,
            y: 0.0,
        },
        radius,
    };
    
    let mut rng = rand::thread_rng();
    let pt_list = gen_point_list(&mut rng, &config, 50);
    let loc_list = locate_n_point(&circle, &pt_list);
    
    let layer = Layer {
        name: "Points".to_string(),
        color_inside: "#04E7336D".to_string(),
        color_outside: "#A954BDE2".to_string(),
        points: pt_list,
        points_location: loc_list,
    };
    
    println!("{}", generate_svg_for_layer(&layer));

}

#[test]
fn test_locate_n_point() {
    let mut rng = rand::thread_rng();
    let cfg = RandConfig{
        x_min: -1.5, x_max: 1.5,
        y_min: -1.5, y_max: 1.5
    };
    let c = Circle{
                center: Point{
                            x: -0.1, 
                            y: -0.1
                            }, 
                radius: 0.8
    };
    let pt_list = vec![
                    Point { x: -1.1621318, y: 1.2528145 },
                    Point { x: -0.27728224, y: 0.40918386 },
                    Point { x: -1.0215106, y: 0.29682744 }
                ];
    // println!("{:?}", pt_list);

    let loc_list = locate_n_point(&c, &pt_list);
    let expected: Vec<PointLocation> = vec![
                        PointLocation::Outside(Point { x: -1.1621318, y: 1.2528145 }, 1.7199509),
                        PointLocation::Inside(Point { x: -0.27728224, y: 0.40918386 }, 0.5391635),
                        PointLocation::Outside(Point { x: -1.0215106, y: 0.29682744 }, 1.0033214)
                    ];

    assert_eq!(loc_list, expected);
    
    // Use a for loop to compare individual elements
    for (actual, expected) in loc_list.iter().zip(expected.iter()) {
        assert_eq!(actual, expected);
    }
}
