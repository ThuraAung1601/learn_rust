// No 3.3
// Arrow 1 Recursion 

fn make_arrow1(size: i32, row_index: &mut i32, arrow: &mut String ) {
    // for _i in 1..=size
    *row_index += 1;
    if *row_index < size*2 {
        let mut star_index: i32 = 0;
        if *row_index <= size {
            // println!("*");
            star_generator(*row_index, &mut star_index, arrow);
        } 
        else {
            let rev_index = (size*2) - *row_index;
            star_generator(rev_index, &mut star_index, arrow);
        }
        arrow.push_str("\n");
        make_arrow1(size, row_index, arrow);
    }
}

fn star_generator(row_index: i32, star_index: &mut i32, arrow: &mut String) {
    // for _j in 1..=_i
    *star_index += 1;
    if *star_index <= row_index {
        arrow.push_str("*");
        star_generator(row_index, star_index, arrow);
    }
}

fn main() {
    // let args: Vec<String> = std::env::args().collect();
    // let size_arg = if args.len() < 2 {""} else {&args[1]};
    // let size: i32 = size_arg.parse().unwrap_or(0);
    
    // let arrow1 = make_arrow1(size);

    let arrow1_size: i32 = 3;
    let mut arrow1 = String::new();
    let mut row_index = 0;
    make_arrow1(arrow1_size, &mut row_index, &mut arrow1);
    print!("{}",arrow1);
}

#[test]
fn test_make_arrow1() {
    let star = 4;
    let expected = "*\n**\n***\n****\n***\n**\n*\n";
    let arrow = make_arrow1(star);

    assert_eq!(expected, arrow);
}