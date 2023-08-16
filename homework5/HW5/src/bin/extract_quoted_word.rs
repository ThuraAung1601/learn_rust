fn extract_quoted_words(text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current_word = String::new();
    let mut is_quoting = false;

    for c in text.chars() {
        match c {
            '*' if !is_quoting => is_quoting = true,
            '*' if is_quoting => {
                if !current_word.is_empty() {
                    words.push(current_word.clone());
                }
                current_word.clear();
                is_quoting = false;
            }
            ' ' if is_quoting => {
                if !current_word.is_empty() {
                    words.push(current_word.clone());
                }
                current_word.clear();
                is_quoting = false;
            }
            _ if is_quoting => current_word.push(c),
            _ => {}
        }
    }
    
    if !current_word.is_empty() {
        words.push(current_word);
    }

    return words;
}

fn main() {
    let input = "C *C++* *Python* Rust*";
    println!("{:?}",extract_quoted_words(input));
}

#[test]
fn test_extract_quoted_words() {
    let expected_output: Vec<String> = vec![];
    assert_eq!(extract_quoted_words(""), expected_output);
    assert_eq!(
    extract_quoted_words("*Python* Rust*"),
        vec!["Python"] // "**", "*C++*", "*Python*"
    );
}
