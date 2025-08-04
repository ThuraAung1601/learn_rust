// No 3.1
// How to run: cargo run --bin star_pattern1 <output-size>

fn star_pattern_1(size:i32){

    // i is for rows: first loop of i is first row 
    for _i in 0..size {
    /* j is for stars: if i is 2, j will be 2 
     meaning that in 2nd row, there will be 2 stars */
        for _j in 0..=_i {
            print!("*")
        }
        println!()
    }

    // the previous loop but in reverse order
    /* start from 1 because we already have n-size stars and
    need to start from (n-1) stars */
    for _i in (1..size).rev() {
        for _j in 0.._i {
                print!("*")
            }
            println!()
        }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let size_arg = if args.len() < 2 {""} else {&args[1]};
    let size: i32 = size_arg.parse().unwrap_or(0);
    
    star_pattern_1(size);
}