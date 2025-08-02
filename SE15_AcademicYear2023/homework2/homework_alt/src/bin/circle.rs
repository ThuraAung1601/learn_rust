fn main(){
    let args: Vec<String> = std::env::args().collect();
    let radius_arg = if args.len() < 2 {""} else {&args[1]};
    
    let radius: f32 = radius_arg.parse().unwrap_or(0.0);
    let pi = 3.1416;
    let circle_area = pi * radius * radius;

    println!("The area of the circle with radius {radius} is {circle_area}");
    
}
