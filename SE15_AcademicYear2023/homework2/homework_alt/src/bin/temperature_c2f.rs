fn main(){
    let args: Vec<String> = std::env::args().collect();
    let cel_arg = if args.len() < 2 {""} else {&args[1]};
    
    let cel: f32 = cel_arg.parse().unwrap_or(0.0);
    
    let fah = ((9.0 * cel)/5.0) + 32.0;

    println!("Temperature at {cel} Celsius is {fah} Fahrenheit");

}