fn quote_list(v: &[&str], c: char) -> Vec<String> {
    v.iter().map(|s| format!("{}{}{}", c, s, c)).collect()
    }

// rewrite with function return
fn quote_list_loop(v: &mut Vec<String>, c: char) {
    for _i in 0..v.len() {
        v[_i] = format!("{}{}{}", c, v[_i], c);
    }
}

fn quote_list_recursion(v: &mut Vec<String>, c: char, _i:usize) {
    if _i < v.len() {
        v[_i] = format!("{}{}{}", c, v[_i], c);
        quote_list_recursion(v, c, _i+1);
    }
}

#[test]
fn test_quotes() {
    assert_eq!(quote_list(&[""; 0], '*'), &[""; 0]);
    assert_eq!(
    quote_list(&["abcd", "xyz"], '*'),
    ["*abcd*", "*xyz*"]);
}

#[test]
fn test_quotes_loop() {
    let mut strings = vec!["abcd".to_string(), "xyz".to_string()];
    let c: char = '*';
    quote_list_loop(&mut strings, c);
    let expected = vec!["*abcd*", "*xyz*"];
    assert_eq!(strings,expected);

    strings = vec!["xyz".to_string()];
    quote_list_loop(&mut strings, c);
    assert_eq!(strings, ["*xyz*"]);
}

#[test]
fn test_quotes_recursion() {
    let mut strings = vec!["abcd".to_string(), "xyz".to_string()];
    let c: char = '*';
    quote_list_recursion(&mut strings, c, 0);
    let expected = vec!["*abcd*", "*xyz*"];
    assert_eq!(strings,expected);
}
