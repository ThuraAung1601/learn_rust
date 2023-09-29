use rand::Rng;
use std::io;
use std::io::Write;
use std::str::FromStr;

#[derive(PartialEq)]
#[derive(Clone)]

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
struct Bound {
    circle1: Circle,
    circle2: Circle,
}

#[derive(Debug)]
struct RandConfig {
    x_min: f32, x_max: f32,
    y_min: f32, y_max: f32,
}

#[derive(Debug)]
#[derive(PartialEq)]
enum PointLocation {
    InsideBoth(Point, f32),
    InsideFirst(Point, f32),
    InsideSecond(Point, f32),
    Outside(Point, f32),
}

fn gen_point_list(rng: &mut impl Rng, config: &RandConfig, num: usize) -> Vec<Point> {
    let mut pt_list: Vec<Point> = Vec::new();
    for _i in 0..num {
        let x = rng.gen_range(config.x_min ..= config.x_max);
        let y = rng.gen_range(config.y_min ..= config.y_max);
       
        pt_list.push(Point{x, y});
    }
    return pt_list;
}

fn locate_n_point2(bound: &Bound, pt_ls: &[Point]) -> Vec<PointLocation> {
    let mut loc_list = Vec::new();
    for pt in pt_ls {
        let dx = pt.x - bound.circle1.center.x;
        let dy = pt.y - bound.circle1.center.y;
       
        let distance = (dx * dx + dy * dy).sqrt();

        if distance <= bound.circle1.radius && distance <= bound.circle2.radius {
           
            loc_list.push(PointLocation::InsideBoth(pt.clone(), distance));
        } 
        else if distance <= bound.circle1.radius {
           
            loc_list.push(PointLocation::InsideFirst(pt.clone(), distance));
        }
        else if distance <= bound.circle2.radius {
           
            loc_list.push(PointLocation::InsideSecond(pt.clone(), distance));
        }
        else {
            loc_list.push(PointLocation::Outside(pt.clone(), distance));
        }
    }
    return loc_list;
}

fn main() {

    // No 4.1
    // let mut rng = rand::thread_rng();
    // let cfg = RandConfig{
    //     x_min: -1.5, x_max: 1.5,
    //     y_min: -1.5, y_max: 1.5
    // };
    // let c1 = Circle{
    //             center: Point{
    //                         x: -0.1, 
    //                         y: -0.1
    //                         }, 
    //             radius: 0.8
    // };
    // let c2 = Circle{
    //     center: Point{
    //                 x: -0.1, 
    //                 y: -0.1
    //                 }, 
    //     radius: 1.6
    // };
    // let b: Bound = Bound {
    //     circle1: c1,
    //     circle2: c2
    // };
    // let pt_list = gen_point_list(&mut rng, &cfg, 5);
    // println!("{:?}", pt_list);

    // let loc_list = locate_n_point2(&b, &pt_list);
    // for loc in loc_list {
    //    println!("{loc:?}");
    // }

    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 9 {
        eprintln!("Usage: {} <min> <max> <x> <y> <radius>", args[0]);
        std::process::exit(1);
    }
    
    let min = f32::from_str(&args[1]).expect("Invalid x_min");
    let max = f32::from_str(&args[2]).expect("Invalid x_max");
    let x1 = f32::from_str(&args[3]).expect("Invalid x_max");
    let y1 = f32::from_str(&args[4]).expect("Invalid y_min");
    let radius1 = f32::from_str(&args[5]).expect("Invalid radius");
    let x2 = f32::from_str(&args[6]).expect("Invalid x_max");
    let y2 = f32::from_str(&args[7]).expect("Invalid y_min");
    let radius2 = f32::from_str(&args[8]).expect("Invalid radius");
    
    let config = RandConfig {
        x_min: min,
        x_max: max,
        y_min: min,
        y_max: max,
    };
    
    let circle1 = Circle {
        center: Point {
            x: x1,
            y: y1,
        },
        radius: radius1,
    };
    
    let circle2 = Circle {
        center: Point {
            x: x2,
            y: y2,
        },
        radius: radius2,
    };

    let mut rng = rand::thread_rng();
    let pt_list = gen_point_list(&mut rng, &config,  1000);
    
    let mut svg = String::new();
    let header = format!("<svg height=\"{}\" width=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">\n", max, max);
    svg.push_str(&header);
    let big_circle1 = format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" stroke=\"black\" stroke-width=\"2\" fill=\"white\" />\n", circle1.center.x, circle1.center.y, circle1.radius);
    let big_circle2 = format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" stroke=\"black\" stroke-width=\"2\" fill=\"white\" />\n", circle2.center.x, circle2.center.y, circle2.radius);
    svg.push_str(&big_circle1);
    svg.push_str(&big_circle2);

    for pt in pt_list {
        let dx1 = pt.x - circle1.center.x;
        let dy1 = pt.y - circle1.center.y;
        // println!("{} {}", dx, dy);
        let distance1 = (dx1 * dx1 + dy1 * dy1).sqrt();

        let dx2 = pt.x - circle2.center.x;
        let dy2 = pt.y - circle2.center.y;
        // println!("{} {}", dx, dy);
        let distance2 = (dx2 * dx2 + dy2 * dy2).sqrt();

        if distance1 <= circle1.radius && distance2 <= circle2.radius {
            // pt.clone() in the PointLocation enum variants is to avoid ownership issues 
            // and ensure that each PointLocation variant contains its own copy of the Point.
            let small_circle = format!("<circle cx=\"{}\" cy=\"{}\" r=\"2\" stroke=\"black\" stroke-width=\"2\" fill=\"green\" />\n", pt.x, pt.y);
            svg.push_str(&small_circle);
        } 
        else if distance1 <= circle1.radius {
            // pt.clone() in the PointLocation enum variants is to avoid ownership issues 
            // and ensure that each PointLocation variant contains its own copy of the Point.
            let small_circle = format!("<circle cx=\"{}\" cy=\"{}\" r=\"2\" stroke=\"black\" stroke-width=\"2\" fill=\"red\" />\n", pt.x, pt.y);
            svg.push_str(&small_circle);
        }
        else if distance2 <= circle2.radius {
            // pt.clone() in the PointLocation enum variants is to avoid ownership issues 
            // and ensure that each PointLocation variant contains its own copy of the Point.
            let small_circle = format!("<circle cx=\"{}\" cy=\"{}\" r=\"2\" stroke=\"black\" stroke-width=\"2\" fill=\"blue\" />\n", pt.x, pt.y);
            svg.push_str(&small_circle);
        }
        else {
            let small_circle = format!("<circle cx=\"{}\" cy=\"{}\" r=\"2\" stroke=\"black\" stroke-width=\"2\" fill=\"purple\" />\n", pt.x, pt.y);
            svg.push_str(&small_circle);
        }
    }
    svg.push_str("</svg>");

    println!("{}", svg);
}
