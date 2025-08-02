fn extract_quoted_words(input: &str) -> Vec<String> {
    let mut v = Vec::new();
    let input = input.to_string();
    let input = input.split(" ");
    for word in input {
        if word.starts_with("*") && word.ends_with("*") {
            v.push(word[1..word.len()-1].to_string())
        }
    }
    return v;
}

fn main() {
    let input = "** C *C++* Python *Rust* Java* **";
    let v = extract_quoted_words(input);
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
