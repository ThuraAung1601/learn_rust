fn count_vowels_v2(s: &str) -> Vec<(String, usize)> {
    let mut v = Vec::new();

    for _w in s.split_whitespace() {
        let mut counter = 0;
        for _c in _w.to_lowercase().chars() {
            if _c == 'a' || _c == 'e' || _c == 'i' || _c == 'o' || _c == 'u' {
                counter += 1;
            }
            else {
                continue;
            }
        }
        // Tuple
        let t = (_w.to_string(), counter);
        v.push(t);
    }
    // return is vector of tuples
    return v;
}

fn main() {
    let s = "ab123 Ef xey45";
    println!("{:?}", count_vowels_v2(s));

}

#[test]
fn test_vowels_count2() {
    assert_eq!(count_vowels_v2(""), []);
    assert_eq!(
    count_vowels_v2("ab12Exey5 7x8U3y5z"),
        [
            ("ab12Exey5".to_string(), 3), // 'a', 'E', 'e'
            ("7x8U3y5z".to_string(), 1) // 'U'
        ]
    );
    assert_eq!(
        count_vowels_v2("ab12Exey5 7x8U 3y5z"),
            [
                ("ab12Exey5".to_string(), 3), // 'a', 'E', 'e'
                ("7x8U".to_string(), 1), // 'U'
                ("3y5z".to_string(), 0) // 
            ]
    );
}
