fn split_grades(g: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    for _i in 0..g.len() {
        if g[_i] == "D" || g[_i] == "F" {
            v2.push(g[_i]);
        }
        else {
            v1.push(g[_i]);
        }
    }
    return (v1, v2);
}

fn main() {
    let grade_list = vec!["B", "F", "A+", "D", "C"];
    println!("{:?}", split_grades(grade_list));
}

#[test]
fn test_split_grades() {
    // testing with blank vector
    assert_eq!(split_grades(vec![]), (vec![], vec![]));

    let expected = (vec!["B", "A+", "C"], vec!["F", "D"]);
    let result = split_grades(vec!["B", "F", "A+", "D", "C"]);
    assert_eq!(result, expected);
}