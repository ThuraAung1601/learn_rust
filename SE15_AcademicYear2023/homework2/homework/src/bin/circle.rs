use clap::{Arg, App};

fn main() {
    let matches = App::new("circle")
            .version("0.1.0")
            .author("Thura Aung")
            .about("calculate the area of a circle")
            .arg(
                Arg::with_name("radius")
                    .value_name("radius")
                    .help("Enter radius for circle area calculation")
                    .required(true)
                    .short("r")
                    .index(1)
            ).get_matches();
    
    let radius_arg = matches.value_of("radius").unwrap().to_string();
    let radius = radius_arg.parse().unwrap_or(0.0);
    
    let pi = 3.1416;
    let circle_area = pi * radius * radius;

    println!("The area of the circle with radius {radius} is {circle_area}");
    
    
}
