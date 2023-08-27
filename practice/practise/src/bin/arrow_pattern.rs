fn arrow_pattern1(size: i32) -> String {
    let mut arrow = String::new();
    for i in 1..=size {
        arrow.push_str(&"*".repeat(i as usize));
        arrow.push('\n');
    }
    for i in (1..size).rev() {
        arrow.push_str(&"*".repeat(i as usize));
        arrow.push('\n');
    }
    return arrow;
}

fn arrow_pattern2(size: i32) -> String {
    let mut arrow = String::new();
    for i in (1..size).rev() {
        arrow.push_str(&" ".repeat(i as usize));
        arrow.push_str(&"*".repeat((size - i) as usize));
        arrow.push('\n');
    }
    for i in 0..size {
        arrow.push_str(&" ".repeat(i as usize));
        arrow.push_str(&"*".repeat((size - i) as usize));
        arrow.push('\n');
    }
    return arrow;
}


fn main(){
    let size = 3;
    println!("{}", arrow_pattern1(size));
    println!("{}", arrow_pattern2(size));
}