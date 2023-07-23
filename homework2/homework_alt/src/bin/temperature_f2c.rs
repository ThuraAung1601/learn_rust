fn main(){
    let args: Vec<String> = std::env::args().collect();
    let fah_arg = if args.len() < 2 {""} else {&args[1]};
    
    let fah: f32 = fah_arg.parse().unwrap_or(0.0);
    
    let cel = (5.0/9.0)*(fah-32.0);

    println!("Temperature at {fah} Fahrenheit is {cel} Celsius");
    
}