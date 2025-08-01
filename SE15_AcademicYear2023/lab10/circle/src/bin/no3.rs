use rand::Rng;
use std::io;
use std::io::Write;
use std::str::FromStr;

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
    // println!("{:?}", pt_list);

    // // No 3.2
    // let loc_list = locate_n_point(&c, &pt_list);
    // for loc in loc_list {
    //    println!("{loc:?}");
    // }

    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 6 {
        eprintln!("Usage: {} <min> <max> <x> <y> <radius>", args[0]);
        std::process::exit(1);
    }
    
    let min = f32::from_str(&args[1]).expect("Invalid x_min");
    let max = f32::from_str(&args[2]).expect("Invalid x_max");
    let x = f32::from_str(&args[3]).expect("Invalid x_max");
    let y = f32::from_str(&args[4]).expect("Invalid y_min");
    let radius = f32::from_str(&args[5]).expect("Invalid radius");
    
    let config = RandConfig {
        x_min: min,
        x_max: max,
        y_min: min,
        y_max: max,
    };
    
    let circle = Circle {
        center: Point {
            x,
            y,
        },
        radius,
    };
    
    let mut rng = rand::thread_rng();
    let pt_list = gen_point_list(&mut rng, &config,  1000);
    
    let mut svg = String::new();
    let header = format!("<svg height=\"{}\" width=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">\n", max, max);
    svg.push_str(&header);
    let big_circle = format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" stroke=\"black\" stroke-width=\"2\" fill=\"white\" />\n", circle.center.x, circle.center.y, circle.radius);
    svg.push_str(&big_circle);

    for pt in pt_list {
        let dx = pt.x - circle.center.x;
        let dy = pt.y - circle.center.y;
        // println!("{} {}", dx, dy);
        let distance = (dx * dx + dy * dy).sqrt();

        if distance <= circle.radius {
            let small_circle = format!("<circle cx=\"{}\" cy=\"{}\" r=\"2\" stroke=\"black\" stroke-width=\"2\" fill=\"red\" />\n", pt.x, pt.y);
            svg.push_str(&small_circle);
        }
        else {
            let small_circle = format!("<circle cx=\"{}\" cy=\"{}\" r=\"2\" stroke=\"black\" stroke-width=\"2\" fill=\"blue\" />\n", pt.x, pt.y);
            svg.push_str(&small_circle);
        }
    }
    svg.push_str("</svg>");

    println!("{}", svg);
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
