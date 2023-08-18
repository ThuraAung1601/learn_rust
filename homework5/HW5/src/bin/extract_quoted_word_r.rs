fn extract_quoted_words_r(quote: &str, result: &mut Vec<String>) {
    if let Some((word, rest)) = quote.split_once(" ") {
        let open_star = word.chars().next().unwrap_or('_');
        let close_star = word.chars().rev().next().unwrap_or('_');
        if open_star == '*' && close_star == '*' {
            let (_, word) = word.split_at(1);
            let (word, _) = word.split_at(word.len() - 1);

            result.push(word.to_string());
        }
        extract_quoted_words_r(rest, result);
    }
}

fn main() {
    let qoute = "C ** *C++* *Java *Python* Rust*";
    let mut result = Vec::new();
    extract_quoted_words_r(qoute, &mut result);
    println!("{:?}", result);
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
