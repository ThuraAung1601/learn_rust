fn main(){
    let args: Vec<String> = std::env::args().collect();
    let p1_arg = if args.len() < 2 {"N/A"} else {&args[1]};
    let p2_arg = if args.len() < 3 {"N/A"} else {&args[2]};
    let p1 = p1_arg.to_string();
    let p2 = p2_arg.to_string();
    
    println!("Player 1: {}\nPlayer 2: {}", p1, p2);
}