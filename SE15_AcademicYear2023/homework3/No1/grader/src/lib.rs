// No 1.1

// to determine the grade from the score
pub fn grade_checker(mark: f32) -> &'static str {
    if mark >= 0.0 && mark <= 49.0 {return "Failed with F";}
    else if mark >= 50.0 && mark <= 60.0 {return "D";}
    else if mark >= 61.0 && mark <= 70.0 {return "C";}
    else if mark >= 71.0 && mark <= 80.0 {return "B";}
    else if mark >= 81.0 && mark <= 94.0 {return "A";}
    else if mark >= 95.0 && mark <= 100.0 {return "Excellent with A+";}
    else {return "Invalid score"} // this is the case < 0 or > 100
}