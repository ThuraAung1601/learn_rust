// No 3.2
// How to run: cargo run --bin star_pattern2 <output-size>

fn star_pattern_2(size:i32){
    // i is for rows, j is for spaces and k is for stars
    // start i from 1 to start with (n-1) spaces
    for _i in (1..size).rev() {
    // since i in the first loop is 1, to avoid 0..=1 start j from 1
        for _j in 1..=_i {
            print!(" ") // try with print!("2") to see patterns
        }
        for _k in _i..size {
            print!("*")
        } println!()
    }
    for _i in 0..size {
        for _j in 0.._i {
            print!(" ")
        }
        for _k in _i..size {
            print!("*")
        } println!()
    }
    
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let size_arg = if args.len() < 2 {""} else {&args[1]};
    let size: i32 = size_arg.parse().unwrap_or(0);
    
    star_pattern_2(size);
}
