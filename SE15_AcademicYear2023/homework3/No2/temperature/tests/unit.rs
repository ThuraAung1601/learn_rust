// No 2.2

use float_cmp::approx_eq; // to check the temperature conversion function

#[test]
fn first_test_with_step_20() {
    let test_cases = vec![
        (0.0, -17.8),
        (20.0, -6.7),
        (40.0, 4.4),
        (60.0, 15.6),
        (80.0, 26.7),
        (100.0, 37.8),
        (120.0, 48.9),
        (140.0, 60.0),
        (160.0, 71.1),
        (180.0, 82.2),
        (200.0, 93.3),
        (220.0, 104.4),
        (240.0, 115.6),
        (260.0, 126.7),
        (280.0, 137.8),
        (300.0, 148.9)];

    for (input, expected) in test_cases {
        // increase epsilon because the output example values shown in the instructions are round up to 0.x 
        assert!(approx_eq!(f32, temperature::fahr2cel(input as i32), expected, epsilon = 0.1));
    }
} 

#[test]
fn second_test_with_step_40() {

    let test_cases = vec![
        (0.0, -17.8),
        (40.0, 4.4),
        (80.0, 26.7),
        (120.0, 48.9),
        (160.0, 71.1),
        (200.0, 93.3),
        (240.0, 115.6),
        (280.0, 137.8)];

    for (input, expected) in test_cases {
        assert!(approx_eq!(f32, temperature::fahr2cel(input as i32), expected, epsilon = 0.1));
    }
}         
 
#[test]
fn third_test_with_reverse() {
      let test_cases = vec![
        (300.0, 148.9),
        (280.0, 137.8),
        (260.0, 126.7),
        (240.0, 115.6),  
        (220.0, 104.4),
        (200.0, 93.3),
        (180.0, 82.2),
        (160.0, 71.1),
        (140.0, 60.0),
        (120.0, 48.9),
        (100.0, 37.8),
        (80.0, 26.7),
        (60.0, 15.6),
        (40.0, 4.4),
        (20.0, -6.7),
        (0.0, -17.8)];

      for (input, expected) in test_cases {
          assert!(approx_eq!(f32, temperature::fahr2cel(input as i32), expected, epsilon = 0.1));
      }
} 