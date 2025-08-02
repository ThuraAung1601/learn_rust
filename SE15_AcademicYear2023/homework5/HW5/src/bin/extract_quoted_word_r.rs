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
fn test_extract_quoted_words() {
    let mut v = Vec::new();
    extract_quoted_words_r("", &mut v);
    assert_eq!(v, Vec::<String>::new());

    let mut v1 = Vec::new();
    extract_quoted_words_r("C ** *C++* *Java *Python* Rust*", &mut v1);

    assert_eq!(
    v1, vec!["", "C++", "Python"] // "**", "*C++*", "*Python*"
    );
}
