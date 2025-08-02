use std::io::Read;
use std::io::Write;
use std::fs::File;

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
struct Point {
    x: f64,
    y: f64,
    color: String,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point {
            x,
            y,
            color: String::new(),
        }
    }
}

fn tag_points(pt_list: Vec<Point>) -> Vec<Point> {
    let mut tagged_points = Vec::new();

    for mut point in pt_list {
        let distance = ((point.x * point.x + point.y * point.y) as f32).sqrt();
        if distance <= 1.0 {
            // Inside the unit circle (custom green tone)
            point.color = "#80FF8080".to_string(); 
        } else {
            // Outside the unit circle (custom red tone)
            point.color = "#FF808080".to_string(); 
        }
        tagged_points.push(point);
    }

    tagged_points
}

fn load_points(reader: impl Read) -> Vec<Point> {
    let mut point_list = Vec::new();

    let mut rdr = csv::ReaderBuilder::new()
                    .has_headers(false)
                    .from_reader(reader);

    for result in rdr.records() {
        let record = result.unwrap();
        if record.len() < 2 {
            continue;
        }
        let x: f64 = record[0].parse().unwrap_or(0.0);
        let y: f64 = record[1].parse().unwrap_or(0.0);
        point_list.push(Point::new(x, y));
    }
    return point_list;
}

fn save_points(writer: impl Write, pt_list: Vec<Point>) {
    let mut wtr = csv::WriterBuilder::new()
                    .has_headers(false)
                    .delimiter(b',')
                    .from_writer(writer);

    for point in pt_list {
        wtr.serialize((point.x, point.y, point.color)).unwrap();
        // wtr.write_record((point.x, point.y, &point.color)).unwrap();
    }

    wtr.flush().unwrap();
}

fn main() {
    let p1 = Point::new(1.2, -0.8);
    let p2 = Point::new(0.2, 0.9);
    let p3 = Point::new(-0.7, -0.7);

    // println!("{:?}", p1);
    // println!("{:?}", p2);
    // println!("{:?}", p3);

    let points = vec![p1, p2, p3];
    
    // no 3.1
    let tagged_points = tag_points(points);
    for point in &tagged_points {
        println!("{}", point.color);
    }

    // // no 3.2
    // let file = File::open("./src/test_pts.csv").expect("Failed to open file");
    // let points = load_points(file);
    // // for point in points {
    // //     println!("{:?}", point);
    // // }
    // let tagged_points = tag_points(points);
    // for point in &tagged_points {
    //     println!("{}", point.color);
    // }

    // no 3.3
    let file = File::create("./src/test_output.csv").expect("Failed to create file");
    save_points(file, tagged_points);
}

#[test]
fn test_point_color() {
    let points = vec![
        Point::new(1.2, -0.8),
        Point::new(0.2, 0.9),
        Point::new(-0.7, -0.7),
    ];
    let tagged_points = tag_points(points);

    for point in &tagged_points {
        let distance = ((point.x * point.x + point.y * point.y) as f32).sqrt();
        if distance <= 1.0 {
            assert_eq!(point.color, "#80FF8080".to_string());
        } else {
            assert_eq!(point.color, "#FF808080".to_string());
        }
    }
}

#[test]
fn test_load_points() {
    let data = "\
            1.2, -0.8\n\
            0.2, 0.9\n\
            0.7, -0.7\n\
            ".as_bytes();

    let points = load_points(data);
    // println!("{:?}", result[0]);
    let result = tag_points(points);

    assert_eq!(result.len(), 3);
    assert_eq!(result[0].color, "#FF808080");
    assert_eq!(result[1].color, "#80FF8080");
    assert_eq!(result[2].color, "#80FF8080");
}

#[test]
fn test_load_save_points() {

    let p1 = Point::new(1.2, -0.8);
    let p2 = Point::new(0.2, 0.9);
    let p3 = Point::new(-0.7, -0.7);

    let points = vec![p1, p2, p3];
    let sv_pts = tag_points(points);

    let sv_file = File::create("./src/test_output.csv").expect("Failed to create file");
    save_points(sv_file, sv_pts.clone());

    let ld_file = File::open("./src/test_output.csv").expect("Failed to open file");
    let ld_pts = load_points(ld_file);
    let ld_pts = tag_points(ld_pts);

    assert_eq!(sv_pts.len(), ld_pts.len());
    let l = sv_pts.len();
    for i in 0..l {
        println!("{:?}", ld_pts[i]);
        assert_eq!(sv_pts[i], ld_pts[i]);
    }
}


use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

struct Args {
    input: Box<dyn BufRead>,
    output: Box<dyn Write>,
}

impl Args {
    fn new(input: Box<dyn BufRead>, output: Box<dyn Write>) -> Self {
        Args { input, output }
    }
}

fn get_args() -> Args {
    let args: Vec<String> = env::args().collect();

    let input: Box<dyn BufRead> = match args.get(1) {
        Some(filename) if filename != "-" => {
            let file = File::open(filename).expect("Failed to open input file");
            Box::new(BufReader::new(file))
        }
        _ => Box::new(BufReader::new(io::stdin())),
    };

    let output: Box<dyn Write> = match args.get(2) {
        Some(filename) if filename != "-" => {
            let file = File::create(filename).expect("Failed to create output file");
            Box::new(file)
        }
        _ => Box::new(io::stdout()),
    };

    Args::new(input, output)
}

fn main() {
    let args = get_args();

    // Use args.input and args.output for reading and writing, respectively

    // Example: Copy lines from input to output
    let mut line = String::new();
    while args.input.read_line(&mut line).unwrap() > 0 {
        args.output.write_all(line.as_bytes()).unwrap();
        line.clear();
    }
}
