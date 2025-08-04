fn join_strings(lst: &[&str], sep: &str) -> String {
    let mut joint_strings = String::new();
    for (index, word) in lst.iter().enumerate() {
        joint_strings.push_str(word);
        if index < lst.len() - 1 {
            joint_strings.push_str(sep);
        }
    }
    joint_strings
}

fn join_num(lst: &[i32], sep: &str) -> String {
    let mut joint_strings = String::new();
    for (index, num) in lst.iter().enumerate() {
        joint_strings.push_str(&num.to_string());
        if index < lst.len() - 1 {
            joint_strings.push_str(sep);
        }
    }
    joint_strings
}

fn join_string2(nv: &[&str], sep: &str) -> String {
    let mut result = String::new();
    let mut iter = nv.iter();
    let v = iter.next();
    if v.is_none() {
        return "".to_string();
    }
    //print!("{}", v.unwrap());
    result.push_str(&v.unwrap());
    while let Some(v) = iter.next() {
        result.push_str(sep);
        result.push_str(&v);
        //print!("{0}{1}", sep, v);
    }
    //println!();
    return result;
}

fn join_num2(nv: &[i32], sep: &str) -> String {
    let mut result = String::new();
    let mut iter = nv.iter();
    let v = iter.next();
    if v.is_none() {
        return "".to_string();
    }
    //print!("{}", v.unwrap());
    result.push_str(&v.unwrap().to_string());
    while let Some(v) = iter.next() {
        result.push_str(sep);
        result.push_str(&v.to_string());
        //print!("{0}{1}", sep, v);
    }
    //println!();
    return result;
}
    
fn pack_number_tuples(l1: &[i32], l2: &[i32]) -> Vec<(i32,i32)> {
    let max_len = l1.len().max(l2.len());
    let result = (0..max_len).map(|i|{
        let val1 = if i < l1.len() {l1[i]} else {0};
        let val2 = if i < l2.len() {l2[i]} else {0};
        (val1, val2)
    }).collect();
    result
}

fn pack_number_tuples_s(l1: &[i32], l2: &[i32]) -> Vec<(i32,i32)> {
    let min_len = l1.len().min(l2.len());
    let result = (0..min_len).map(|i|{
        (l1[i], l2[i])
    }).collect();
    result
}

fn main() {
    let patterns1 = ["C", "Rust", "C++", "Python"];
    println!("{}", join_strings(&patterns1, ", "));

    let patterns1 = ["C", "Rust", "C++", "Python"];
    println!("{}", join_string2(&patterns1, ", "));

    let patterns2 = [5, 10, -1, 2];
    println!("{}", join_num(&patterns2, ", "));

    let patterns2 = [5, 10, -1, 2];
    println!("{}", join_num2(&[5, 10, -1, 2], ", "));

    let num_lst1 = pack_number_tuples(&[5, 1, 4], &[2, 3]);
    println!("{:?}", num_lst1);

    let num_lst2 = pack_number_tuples_s(&[5, 1, 4], &[2, 3]);
    println!("{:?}", num_lst2);
}


#[test]
fn test_join_strings1() {
    assert_eq!(join_strings(&[], ","), "");
    assert_eq!(join_strings(&["C"], ","), "C");

    let patterns = ["C", "Rust", "C++", "Python"];
    assert_eq!(join_strings(&patterns, ", "), "C, Rust, C++, Python");
    assert_eq!(join_strings(&patterns, ";;"), "C;;Rust;;C++;;Python");
}

#[test]
fn test_join_numbers2() {
    assert_eq!(join_num(&[], ","), "");
    assert_eq!(join_num(&[25], ","), "25");
    let patterns = [5, 10, -1, 2];
    assert_eq!(join_num(&patterns, ", "), "5, 10, -1, 2");
    assert_eq!(join_num(&patterns, ";;"), "5;;10;;-1;;2");
}

#[test]
fn test_join_strings3() {
    assert_eq!(pack_number_tuples(&[], &[]), []);
    assert_eq!(pack_number_tuples(&[1], &[]), [(1, 0)]);
    assert_eq!(pack_number_tuples(&[], &[2, 3]), [(0, 2), (0, 3)]);
    assert_eq!(
    pack_number_tuples(&[5, 1, 4], &[2, 3]),
    [(5, 2), (1, 3), (4, 0)]
    );
}

#[test]
fn test_join_strings4() {
    assert_eq!(pack_number_tuples_s(&[], &[]), []);
    assert_eq!(pack_number_tuples_s(&[1], &[]), []);
    assert_eq!(pack_number_tuples_s(&[], &[2, 3]), []);
    assert_eq!(
    pack_number_tuples_s(&[5, 1, 4], &[2, 3]), [(5, 2), (1, 3)]
    );
}