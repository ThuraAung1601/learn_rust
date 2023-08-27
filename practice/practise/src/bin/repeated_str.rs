fn main(){
    let args: Vec<String> = std::env::args().collect();
    let name_arg = if args.len() < 2 {""} else {&args[1]};
    let name = name_arg.to_string();
    
    let greeting = format!("* Hello, {} *", name);

    let stars = "*".repeat(greeting.len());

    // println!("{}", stars);
    // println!("{}", greeting);
    // println!("{}", stars);
    
    let to_print = format!("{}\n{}\n{}",stars, greeting, stars);
    println!("{}", to_print);
}
