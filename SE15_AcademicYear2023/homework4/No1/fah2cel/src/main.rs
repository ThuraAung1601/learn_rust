// No. 1
// cargo run <fah1> <fah2> .... <fahN>
// cargo test

// 1.1 Ordinary loop
fn fahr_to_cel_v(v: &[f32]) -> Vec<f32> {
    // Using reference value instead of cloning makes less computational cost
    let mut x = Vec::new();
    for _i in v {
        // (5.0/9.0)*(fahr - 32.0)
        x.push((5.0/9.0)*(_i - 32.0));
    }
    return x;
}

// 1.2 Recursion
fn fahr_to_cel_v_recursion(v: &mut Vec<f32>, _i: usize) -> Vec<f32> {
    // Modify the original vector v
    if _i < v.len() {
        v[_i] = (5.0 / 9.0) * (v[_i] - 32.0);
        fahr_to_cel_v_recursion(v, _i+1);
    }
    return v.to_vec();
}

fn main(){
    let v_args: Vec<String> = std::env::args().collect();
    let mut v1 = Vec::new();
    
    // Parse it to numerical values
    for i in 1..v_args.len() {
        // if input is character, it will skip
        match v_args[i].parse::<f32>() {
            Ok(value) => v1.push(value),
            Err(_) => {
                eprintln!("Invalid input: \"{0}\" is not a valid input, skip \"{0}\"", v_args[i]);
            }
        }
    }
    // Input vector from user
    println!("Fahr degrees: {:?}",v1);

    // In ordinary loop, I used the reference value
    println!("Celcius degrees w/ loop: {:?}",fahr_to_cel_v(&v1));

    // In recursion, I used v1.clone() to prevent original value of v1
    println!("Celcius degrees w/ recursion: {:?}",fahr_to_cel_v_recursion(&mut v1.clone(), 0));
    
    // To check the original vector is unchanged.
    // println!("Fahr degrees: {:?}",v1);
}

// Test 1 without values
#[test]
fn test1_fahr_to_cel_v(){
    let v1: Vec<f32> = vec![];
    let expected: Vec<f32> = vec![];

    // test with loop function
    assert_eq!(fahr_to_cel_v(&v1),expected);

    // test with recursive function
    assert_eq!(fahr_to_cel_v_recursion(&mut v1.clone(), 0),expected);
}

// Test 2 with positive values
#[test]
fn test2_fahr_to_cel_v(){
    let v1: Vec<f32> = vec![0.,20.,40.];
    let expected: Vec<f32> = vec![-17.777779, -6.666667, 4.4444447];

    // test with loop function
    assert_eq!(fahr_to_cel_v(&v1),expected);

    // test with recursive function
    assert_eq!(fahr_to_cel_v_recursion(&mut v1.clone(), 0),expected);
}

// Test 3 with negative values
#[test]
fn test3_fahr_to_cel_v(){
    let v1: Vec<f32> = vec![-0.,-20.,-40.];
    let expected: Vec<f32> = vec![-17.777779, -28.88889, -40.0];

    // test with loop function
    assert_eq!(fahr_to_cel_v(&v1),expected);

    // test with recursive function
    assert_eq!(fahr_to_cel_v_recursion(&mut v1.clone(), 0),expected);
}
