// No 3.3
// Arrow 2 Recursion 
// cargo run --bin arrow2_recursion 

fn make_arrow2(size: i32, row_index: &mut i32, arrow: &mut String ) {
    *row_index += 1;
    if *row_index < size*2 {
        let mut star_index: i32 = 0;
        let mut space_index: i32 = *row_index;
        if *row_index <= size {
            // println!("*");
            space_generator(size, &mut space_index, arrow);
            star_generator(*row_index, &mut star_index, arrow);
        } 
        else {
            let mut rev_index = size;
            let mut star_rev_index = *row_index;
            space_generator(*row_index, &mut rev_index, arrow);
            star_generator(size*2, &mut star_rev_index , arrow);
        }
        arrow.push_str("\n");
        make_arrow2(size, row_index, arrow);
    }
}

fn space_generator(size: i32, space_index: &mut i32, arrow: &mut String) {
    *space_index += 1;
    if *space_index <= size {
        arrow.push_str(" ");
        space_generator(size, space_index, arrow);
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
    // let arrow2_size = make_arrow2(size);

    let arrow2_size: i32 = 3;
    let mut arrow2 = String::new();
    let mut row_index = 0;
    make_arrow2(arrow2_size, &mut row_index, &mut arrow2);
    print!("{}",arrow2);
}


#[test]
fn test_make_arrow2_recur() {  
      let star_num = 3;
      let mut index = 0;
      let mut arrow = String::new();
      let expected = "  *\n **\n***\n **\n  *\n";
      make_arrow2(star_num, &mut index, &mut arrow);
      assert_eq!(expected, arrow);
}

#[test]
fn test_arrow2_star_generator_with_zero() {
        
        let star_num = 0;
        let mut  arrow = String::new();
        let mut index = 0;
        let expected = "";

        star_generator(star_num, &mut index, &mut arrow);

        assert_eq!(expected, arrow);
}

#[test]
fn test_arrow2_star_generator_with_two() {
        
        let star_num = 2;
        let mut  arrow = String::new();
        let mut index = 0;
        let expected = "**";

        star_generator(star_num, &mut index, &mut arrow);

        assert_eq!(expected, arrow);
}

#[test]
fn test_arrow2_space_generator() {
        
        let space_num = 1;
        let mut  arrow = String::new();
        let mut index = 0;
        let expected = " ";

        space_generator(space_num, &mut index, &mut arrow);

        assert_eq!(expected, arrow);
}
