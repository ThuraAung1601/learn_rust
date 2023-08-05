use assert_cmd::Command;

// No 3.1
// How to run: cargo run <output-size>

fn make_arrow1(size:i32) -> String{ 
    let mut arrow = String::new();
    // i is for rows: first loop of i is first row 
    for _i in 0..size {
    /* j is for stars: if i is 2, j will be 2 
     meaning that in 2nd row, there will be 2 stars */
        for _j in 0..=_i {
            arrow.push_str("*");
        }
        arrow.push_str("\n");
    }

    // the previous loop but in reverse order
    /* start from 1 because we already have n-size stars and
    need to start from (n-1) stars */
    for _i in (1..size).rev() {
        for _j in 0.._i {
                arrow.push_str("*");
            }
            arrow.push_str("\n");
        }
    return arrow;
}

// No 3.2
// How to run: cargo run --bin star_pattern2 <output-size>

fn make_arrow2(size:i32) -> String {
    let mut arrow = String::new();
    // i is for rows, j is for spaces and k is for stars
    // start i from 1 to start with (n-1) spaces
    for _i in (1..size).rev() {
    // since i in the first loop is 1, to avoid 0..=1 start j from 1
        for _j in 1..=_i {
            arrow.push_str(" "); // try with print!("2") to see patterns
        }
        for _k in _i..size {
            arrow.push_str("*");
        } 
        arrow.push_str("\n");
    }

    for _i in 0..size {
        for _j in 0.._i {
            arrow.push_str(" ");
        }
        for _k in _i..size {
            arrow.push_str("*");
        } 
        arrow.push_str("\n");
    }
    return arrow;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let size_arg = if args.len() < 2 {""} else {&args[1]};
    let size: i32 = size_arg.parse().unwrap_or(0);
    
    let arrow1 = make_arrow1(size);
    print!("{}", arrow1);

    let arrow2 = make_arrow2(size);
    print!("{}", arrow2);
}

#[test]
fn test_make_arrow1() {

}