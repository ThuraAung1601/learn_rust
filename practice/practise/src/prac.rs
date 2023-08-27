fn main(){
    let args: Vec<String> = std::env::args().collect();
    
    let n1_arg = if args.len() < 2 {""} else {&args[1]};
    let n2_arg = if args.len() < 2 {""} else {&args[2]};

    let n1 = n1_arg.parse().unwrap_or(0);
    let n2 = n2_arg.parse().unwrap_or(0);

    let sum = n1 + n2;
    println!("The sum is {sum}.");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n1_arg = if args.len() < 2 {""} else {&args[1]};
    let start = n1_arg.parse().unwrap_or(0);

    let n2_arg = if args.len() < 3 {""} else {&args[2]};
    let end = n2_arg.parse().unwrap_or(0);

    let n3_arg = if args.len() < 4 {""} else {&args[3]};
    let step = n3_arg.parse().unwrap_or(0);

    println!("Input\tOutput");
    for i in (start..end).step_by(step){
        println!("{:>3.1}\t{:>5.1}", i, i as f32 * 2.2);
    }
}


fn main() {
    let radius = 3.0;
    let pi = 3.142;
    let area = radius*radius*pi;
    println!("{area}.");
}

fn main() {
    let size = 3;
    let mut stars1 = String::new();

    for _i in 1..=size {
        stars1.push_str(&"*".repeat(_i));
        stars1.push('\n');
    }
    for _i in (1..size).rev() {
        stars1.push_str(&"*".repeat(_i));
        stars1.push('\n');
    }
    println!("{stars1}");

    let mut stars2 = String::new();
    for _i in (1..size).rev() {
        stars2.push_str(&" ".repeat(_i));
        stars2.push_str(&"*".repeat(size - _i));
        stars2.push('\n');
    }
    for _i in 0..size {
        stars2.push_str(&" ".repeat(_i));
        stars2.push_str(&"*".repeat(size - _i));
        stars2.push('\n');
    }
    println!("{stars2}")
}

fn grade_checker(g: i32) -> String {
    let result: &str;
    if g < 0 || g > 100 {
        result = "Invalid";
    }
    else if g == 100 {
        result = "Full marks.";
    }
    else if g < 100 && g > 50 {
        result = "OK";
    }
    else {
        result = "Practice more.";
    }
    return result.to_string();
}

fn main(){
    println!("{}", grade_checker(100));
}

fn count_neg(v: &[i32]) -> usize {
    let count = v.iter().filter(|&&x| x < 0).count();
    return count;
}

fn count_neg_loop(v: &[i32]) -> i32 {
    let mut count = 0;
    for i in v{
        if *i < 0 {
            count += 1;
        }
    }
    return count;
}

fn count_neg_r(v: &[i32], count: i32) -> i32 {
    if v.len() == 0 {
        return count;
    }
    else {
        if v[0] < 0 {
            count_neg_r(&v[1..], count+1)
        }
        else {
            count_neg_r(&v[1..], count)
        }
    }
}

fn collect_neg(v: &[i32]) -> Vec<&i32> {
    let neg: Vec<_> = v.iter().filter(|&&x| x < 0).collect();
    return neg.to_vec();
}

fn collect_neg_loop(v: &[i32]) -> Vec<i32> {
    let mut neg = Vec::new();
    for i in v {
        if *i < 0 {
            neg.push(*i);
        }
        else {
            continue;
        }
    }
    return neg;
}

fn collect_neg_r(v: &[i32], neg: &mut Vec<i32>) -> Vec<i32> {
    if v.len() == 0 {
        return neg.to_vec();
    }
    else {
        if v[0] < 0 {
            neg.push(v[0]);
            collect_neg_r(&v[1..], neg)
        }
        else {
            collect_neg_r(&v[1..], neg)
        }
    }

}

fn main(){
    let v = [1, -2, -3, 4, 5, -6];

    println!("{}", count_neg(&v));
    println!("{}", count_neg_loop(&v));
    println!("{}", count_neg_r(&v, 0));

    println!("{:?}", collect_neg(&v));
    println!("{:?}", collect_neg_loop(&v));
    println!("{:?}", collect_neg_r(&v, &mut Vec::new()));
}

fn main() {
    let input = "H3ll0 w0r1d.";

    // counter
    let mut count = 0; 
    for _c in input.chars() {
        if _c.is_digit(10) {
            count += 1;
        }
    }
    println!("{count}");

    // collector
    let mut v = Vec::new(); // Vec<(&str, i32)>
    for _w in input.split(" "){
        let mut count = 0;
        for _c in _w.chars() {
            if _c.is_digit(10) {
                count += 1;
            }
        }
        v.push((_w, count));
    }
    println!("{:?}", v);
}

fn count_vowel(s: &str) -> i32 {
    let mut count = 0;
    for _c in s.to_lowercase().chars() {
        match _c {
            'a' | 'e' | 'i' | 'o' | 'u' => count += 1,
            _ => continue,
        }
    }
    return count
}

fn count_vowel_r(s: &str) -> i32 {
    if s.len() == 0 {
        return 0;
    }
    else {
        // let s = s.to_string();
        let _c = s.to_lowercase().chars().next().unwrap();
        let rest = &s[1..];
        let count = count_vowel_r(rest);
        if  _c == 'a' || _c == 'e' || _c == 'i' || _c == 'o' || _c == 'u' {
            return count+1;
        }
        else {
            return count;
        }
    }
}

fn main() {
    let input = "Hello world!";
    println!("{}", count_vowel(input));
    println!("{}", count_vowel_r(input));
}
