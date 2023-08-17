fn extract_recursive(qoute: &str, result: &mut Vec<String>) -> Vec<String> {
    if let Some((word, rest)) = qoute.split_once(" ") {
        let open_star = word.chars().next().unwrap_or('_');
        let close_star = word.chars().rev().next().unwrap_or('_');

        if open_star == '*' && close_star == '*' {
            let (_, word) = word.split_at(1);
            let (word, _) = word.split_at(word.len() - 1);

            result.push(word.to_string());
        }

        let v = extract_recursive(rest, result);
        return v;
    }
    else {
        let v = Vec::<String>::new();
        return v;
    }
}

fn main() {
    let qoute = "C ** *C++* *Java *Python* Rust*      ";
    let mut result = Vec::new();
    extract_recursive(qoute, &mut result);
    println!("{:?}", result);
}
