// No. 2
// cargo run <score1> <score2> .... <score3>
// cargo test

fn grade_checker(mark: f32) -> &'static str {
    if mark >= 0.0 && mark <= 49.0 {return "Failed with F";}
    else if mark >= 50.0 && mark <= 60.0 {return "D";}
    else if mark >= 61.0 && mark <= 70.0 {return "C";}
    else if mark >= 71.0 && mark <= 80.0 {return "B";}
    else if mark >= 81.0 && mark <= 94.0 {return "A";}
    else if mark >= 95.0 && mark <= 100.0 {return "Excellent with A+";}
    else {return "Invalid score"} // this is the case < 0 or > 100
}

// No. 2.1
fn make_grades(s: &[f32]) -> Vec<String> {
    let mut x = Vec::new();
    for _e in s {
        // de-referencing element _e because input is f32 and _e is reference value &f32
        x.push(grade_checker(*_e).to_string());
    }
    return x;
}

// No. 2.2
fn make_grades_recursion(s: Vec<f32>, g: &mut Vec<String>, _i: usize) -> Vec<String> {
   if _i < s.len() {
        g.push(grade_checker(s[_i]).to_string());
        make_grades_recursion(s, g, _i+1);
   }
    return g.clone();
}

fn main() {
    let v_args: Vec<String> = std::env::args().collect();
    let mut scores = Vec::new();

    for _i in 1..v_args.len() {
        // if input is character, it will skip
        match v_args[_i].parse::<f32>() {
            Ok(value) => scores.push(value),
            Err(_) => {
                eprintln!("Invalid input: \"{0}\" is not a valid input, skip \"{0}\"", v_args[_i]);
            }
        }
    }
    
    // Input vector from user
    println!("scores: {:?}",scores);

    // In ordinary loop, I used the reference value
    println!("grades w/ loop: {:?}",make_grades(&scores));

    // In recursion, I have to initialize new vector with string
    println!("grades w/ recursion: {:?}", make_grades_recursion(scores, &mut Vec::new(), 0));
}

// Test 1 without values
#[test]
fn test1_make_grades(){
    let v1: Vec<f32> = vec![];
    let expected: Vec<String> = vec![];

    // test with loop function
    assert_eq!(make_grades(&v1),expected);

    // test with recursive function
    assert_eq!(make_grades_recursion(v1, &mut Vec::new(), 0),expected);
}

// Test 2 with values
#[test]
fn test2_make_grades(){
    let v1: Vec<f32> = vec![100., 80., 40.];
    let expected: Vec<String> = vec!["Excellent with A+".to_string(), "B".to_string(), "Failed with F".to_string()];

    // test with loop function
    assert_eq!(make_grades(&v1),expected);

    // test with recursive function
    assert_eq!(make_grades_recursion(v1, &mut Vec::new(), 0),expected);
}

// Test 3 with invalid values
#[test]
fn test3_make_grades(){
    let v1: Vec<f32> = vec![110., -80., 90.];
    let expected: Vec<String> = vec!["Invalid score".to_string(), "Invalid score".to_string(), "A".to_string()];

    // test with loop function
    assert_eq!(make_grades(&v1),expected);

    // test with recursive function
    assert_eq!(make_grades_recursion(v1, &mut Vec::new(), 0),expected);
}
