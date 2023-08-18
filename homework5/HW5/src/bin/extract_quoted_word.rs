fn extract_quoted_words(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    for mut _w in s.split_whitespace() {
        let open_star = _w.chars().next().unwrap_or('_');
        let close_star = _w.chars().rev().next().unwrap_or('_');

        if open_star == '*' && close_star == '*' {
            // Split open star and word
            (_, _w) = _w.split_at(1);
            // Split close star and word
            (_w, _) = _w.split_at(_w.len()-1);
            result.push(_w.to_string());
        }
    }
    return result;
}


fn main() {
    let s = "C ** *C++* *Java *Python* Rust*";
    let v = extract_quoted_words(s);
    println!("{:?}", v);
}

#[test]
fn test_extract_quoted_words() {
    assert_eq!(extract_quoted_words(""), Vec::<String>::new());
    assert_eq!(
    extract_quoted_words("C ** *C++* *Java *Python* Rust*"),
    ["", "C++", "Python"] // "**", "*C++*", "*Python*"
    );
}
