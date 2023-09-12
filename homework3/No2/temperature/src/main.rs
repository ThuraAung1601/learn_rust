// No 2.1
// How to run: cargo run -- <start> <end> <step>

fn main() {
    let args:Vec<String> = std::env::args().collect();

    let start_arg = if args.len() < 2 {""} else {&args[1]};
    let end_arg = if args.len() < 3 {""} else {&args[2]};
    let step_arg = if args.len() < 4 {""} else {&args[3]};
    
    let start: i32 = start_arg.parse().unwrap_or(0);
    let end: i32 = end_arg.parse().unwrap_or(0);
    let mut step: usize = step_arg.parse().unwrap_or(0);
    
    // iterator used in .step_by() cannot be 0 since step_by(1) means 1 step and if it is step_by(0), it means there is no step.
    // thread 'main' panicked at 'assertion failed: step != 0'
    let mut start: i32 = start_arg.parse().unwrap_or(0);
    let mut end: i32 = end_arg.parse().unwrap_or(0);
    let mut step: usize = step_arg.parse().unwrap_or(0);

    if start == 0 || end == 0 || step == 0 {
        (start, end, step) = (1, 1, 1);
    }
    
    println!("Fahr Celcius");
    if start <= end {
        // Ref: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.step_by
        for fahr in (start..=end).step_by(step){
            let cel: f32 = temperature::fahr2cel(fahr);
           println!("{:>4} {:>7.1}",fahr, cel);
        }
    }
    else {
        // Ref: https://users.rust-lang.org/t/reverse-for-loops/53856/2 
        for fahr in (end..=start).rev().step_by(step){
            let cel: f32 = temperature::fahr2cel(fahr);
            println!("{:>4} {:>7.1}",fahr, cel);
        }
    }
}
