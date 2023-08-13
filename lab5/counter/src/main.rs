fn count_digits(input_str: &str) -> usize {
    let mut counter = 0;
    for _c in input_str.chars() {
        // check whether _c is base 10 digit or not ?
        if _c.is_digit(10) {
            counter += 1;
        }
    }
    return counter;
}

fn count_digits_r(input_str: &str) -> usize {
        if input_str.is_empty() {
            return 0;
        }
        else {
        let _c = input_str.chars().next().unwrap();
        let rest_string = &input_str[_c.len_utf8()..];
        let mut counter = count_digits_r(rest_string);
        if _c.is_digit(10) {
                return counter + 1;
            }
        else {
            return counter;
        }
    }
}

fn count_digits_v2(input_str: &str) -> Vec<usize> {
    let mut v = Vec::new();
    for _w in input_str.split_whitespace() {
        let mut counter = 0;
        for _c in _w.chars() {
            if _c.is_digit(10) {
                counter += 1;
            }
        }
        v.push(counter);
    }
    return v;
}

fn main() {
    let v = "H3ll0 W0r1d";
    let c1 = count_digits(v);
    let c2 = count_digits_v2(v);

    println!("{}", c1);
    println!("{:?}", c2);

    count_digits_r(v);
    println!("{}", count_digits_r(v));
}

#[test]
fn test_digits_count1() {
assert_eq!(count_digits(""), 0);
assert_eq!(count_digits("abcd"), 0);
assert_eq!(count_digits("ab12xy5 7x83y5z"), 7);

assert_eq!(count_digits_r(""), 0);
assert_eq!(count_digits_r("abcd"), 0);
assert_eq!(count_digits_r("ab12xy5 7x83y5z"), 7);

assert_eq!(count_digits_v2(""), []);
assert_eq!(count_digits_v2("abcd"), [0]);
assert_eq!(count_digits_v2("ab12xy5 7x83y5z"), vec![3, 4]);
}
