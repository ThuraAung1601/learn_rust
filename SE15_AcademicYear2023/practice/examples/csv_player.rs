use std::fs::File;
use std::io::Write;
use std::io::BufRead;
use std::io::BufReader;

use rand::Rng;

#[derive(Debug)]
enum Location {
    Inside(f32),
    Outside(f32)
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
    location: Location
}

struct Layer {
    point: Vec<Point>,
    color: String
}

fn generate_pts<R: Rng>(rng: &mut R, n: usize) -> Vec<(f32, f32)> {
    let mut pt_ls = Vec::new();
    for _i in 0..n {
        let x = rng.gen_range(-1. ..= 1.);
        let y = rng.gen_range(-1. ..= 1.);
        pt_ls.push((x, y));
    }
    pt_ls
}

fn filter_pts(pt_ls: &[(f32, f32)]) -> Vec<Point> {
    let mut location_ls = Vec::new();
    for pt in pt_ls {
        let distance = (pt.0*pt.0) + (pt.1*pt.1);
        if distance <= 1. {
            location_ls.push(Point {
                x: pt.0,
                y: pt.1,
                location: Location::Inside(distance)
            });
        }
        else {
            location_ls.push(Point{
                x: pt.0,
                y: pt.1,
                location: Location::Outside(distance)
            });
        }
    }
    location_ls
}

fn main() {
    // writing to csv
    let args: Vec<String> = std::env::args().collect();
    let n = args[1].parse::<usize>().unwrap_or(0);
    let mut rng = rand::thread_rng();
    let generated = generate_pts(&mut rng, n);
    // println!("{:?}", filter_pts(&generated));
    let mut file = File::create("test.csv").expect("Cannot Create.");
    let filtered_ls = filter_pts(&generated);
    for pt in filtered_ls {
        writeln!(file, "{}, {}, {:?}", pt.x, pt.y, pt.location);
    }

    // reading from csv
    let mut pt_ls = Vec::new();
    let mut input = File::open("input.csv").expect("Cannot Open");
    let reader = BufReader::new(input);
    for line in reader.lines() {
        // println!("{:?}", line); // Result
        // println!("{}", line.unwrap());
        let line = line.unwrap();
        let part: Vec<_> = line.split(", ").collect();
        let x = part[0].parse::<f32>().unwrap_or(0.0);
        let y = part[1].parse::<f32>().unwrap_or(0.0);
        pt_ls.push((x, y));
    }
    let filtered_ls = filter_pts(&pt_ls);
    println!("{:?}", filtered_ls);
}