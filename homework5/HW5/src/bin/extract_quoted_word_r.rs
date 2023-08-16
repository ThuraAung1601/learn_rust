fn extract_quoted_words_r(text: &str) -> Vec<String> {
    fn extract_recursive(text: &str, mut current_word: String, is_quoting: bool, words: &mut Vec<String>) {
        if let Some(c) = text.chars().next() {
            match c {
                '*' if !is_quoting => extract_recursive(&text[1..], current_word, true, words),
                '*' if is_quoting => {
                    if !current_word.is_empty() {
                        words.push(current_word.clone());
                    }
                    extract_recursive(&text[1..], String::new(), false, words);
                }
                ' ' if is_quoting => {
                    if !current_word.is_empty() {
                        words.push(current_word.clone());
                    }
                    extract_recursive(&text[1..], String::new(), false, words);
                }
                _ if is_quoting => {
                    current_word.push(c);
                    extract_recursive(&text[1..], current_word, is_quoting, words);
                }
                _ => extract_recursive(&text[1..], current_word, is_quoting, words),
            }
        } else {
            if !current_word.is_empty() {
                words.push(current_word);
            }
        }
    }

    let mut words = Vec::new();
    extract_recursive(text, String::new(), false, &mut words);
    words
}

fn main() {
    let input = "C *C++* *Python* Rust*";
    println!("{:?}",extract_quoted_words_r(input));
}

#[test]
fn test_extract_quoted_words() {
    let expected_output: Vec<String> = vec![];
    assert_eq!(extract_quoted_words(""), expected_output);
    assert_eq!(
        extract_quoted_words_r("*Python* Rust*"),
        vec!["Python"] // "**", "*C++*", "*Python*"
    );
}
