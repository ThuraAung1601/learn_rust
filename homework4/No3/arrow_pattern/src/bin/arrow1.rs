// No 3.1
// How to run: cargo run --bin arrow1 <output-size>

fn make_arrow1(size:i32) -> String{ 
    let mut arrow = String::new();
    // i is for rows: first loop of i is first row 
    for _i in 1..=size {
    /* j is for stars: if i is 2, j will be 2 
     meaning that in 2nd row, there will be 2 stars */
        for _j in 1..=_i {
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let size_arg = if args.len() < 2 {""} else {&args[1]};
    let size: i32 = size_arg.parse().unwrap_or(0);
    
    let arrow1 = make_arrow1(size);
    print!("{}", arrow1);
}

// Test 
#[test]
fn test_make_arrow1() {
    let star = 4;
    let expected = "*\n**\n***\n****\n***\n**\n*\n";
    let arrow = make_arrow1(star);

    assert_eq!(expected, arrow);
}
