fn extract_quote(input: &str) -> Vec<String> {
    let mut v = Vec::new();
    let input = input.to_string();
    let words = input.split(" ");
    for word in words {
        if word.starts_with("*") && word.ends_with("*") {
            v.push(word[1..word.len()-1].to_string())
        }
    }
    return v;
}

fn extract_quote_r(input: &str, v: &mut Vec<String>) -> Vec<String> {
    if input.len() == 0 {
        return v.to_vec();
    }
    else {
        let input = input.to_string();
        let words: Vec<&str> = input.split(" ").collect();
        if words[0].starts_with("*") && words[0].ends_with("*") {
            let quoted = words[0];
            v.push(quoted[1..quoted.len()-1].to_string());
        }
        let rest = words[1..].join(" ");
        return extract_quote_r(rest.as_str(), v);
    }
}

fn main() {
    let input = "** C *C++* Python *Rust* Java* **";
    // let input = "";
    let v = extract_quote_r(input, &mut Vec::<String>::new());
    println!("{:?}", v);
}

#[test]
fn test_extract_quotes() {
    let input = "";
    assert_eq!(extract_quote(input), Vec::<String>::new());

    let input = "** C *C++* Python *Rust* Java* **";
    assert_eq!(extract_quote(input), vec!["", "C++", "Rust", ""]);

    let mut v = Vec::<String>::new();
    let input = "";
    assert_eq!(extract_quote_r(input, &mut v), Vec::<String>::new());
    
    let input = "** C *C++* Python *Rust* Java* **";
    assert_eq!(extract_quote_r(input, &mut v), vec!["", "C++", "Rust", ""]);

}