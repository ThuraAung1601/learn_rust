fn main() {
    // let age = 32; // error - immutable variable
    let mut age = 32;
    {
        let year = 2024;
        println!("In {year} I am {age} years old");
    }
    {
        let year = 2024; // need to declare again because of the scope
        let study_year = 1;
        println!("in {year} i'm studying in year {study_year}");
        // let study_year = 1; // error - declare first
    }

    age = age + 1; 
    println!("now i'm {age} years old");
}