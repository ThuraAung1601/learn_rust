// 'static is a life time parameter
// because just "A" is just String literal
fn grade_checker(mark: f32) -> &'static str {
    if mark >= 0.0 && mark <= 49.0 {return "F";}
    else if mark >= 50.0 && mark <= 60.0 {return "D";}
    else if mark >= 61.0 && mark <= 70.0 {return "C";}
    else if mark >= 71.0 && mark <= 80.0 {return "B";}
    else if mark >= 81.0 && mark <= 94.0 {return "A";}
    else if mark >= 95.0 && mark <= 100.0 {return "A+";}
    else {return "Invalid score"} // this is the case < 0 or > 100
}

fn split_scores(s: Vec<f32>) -> (Vec<(String, f32)>, Vec<(String, f32)>) {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    for _i in 0..s.len() {
        let g = grade_checker(s[_i]);
        if g == "D" || g == "F" {
            v1.push((g.to_string(), s[_i]));
        }
        else {
            v2.push((g.to_string(), s[_i]));
        }
    }
    return (v1, v2);
}

fn main() {
    let scores_list = vec![75., 42., 98., 54., 63.];
    println!("{:?}", split_scores(scores_list));
}

#[test]
fn test_split_scores() {
    // testing with blank vector
    assert_eq!(split_scores(vec![]), (vec![], vec![]));

    let expected = (vec![("F".to_string(), 42.0), ("D".to_string(), 54.0)], vec![("B".to_string(), 75.0), ("A+".to_string(), 98.0), ("C".to_string(), 63.0)]);
    let result = split_scores(vec![75., 42., 98., 54., 63.]);
    assert_eq!(result, expected);
}