// No 1.1
// How to run: cargo run -- <marks>

fn main() {
    let args:Vec<String> = std::env::args().collect();
    let mark_arg = if args.len() < 2 {""} else {&args[1]};
    let mark: f32 = mark_arg.parse().unwrap_or(0.0);
    let grade: &str = grader::grade_checker(mark);
    println!("{}",grade);
}

// if you cargo run <negative>, it will give error
// if you run -- <negative-number>, it will pass
