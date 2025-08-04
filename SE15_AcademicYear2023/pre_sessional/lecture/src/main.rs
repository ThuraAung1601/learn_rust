fn main() {
    // println!("Hello, world!");
    // Rust is statically type language
    // container for str and keeps ownership
    let name: String = "June".to_string();

    // reference 
    let occupation: &str = "Student";

    // explictly declare
    // mut means mutable, able to change
    let mut age: i32 = 23;

    let _age = 21;

    println!("{}", name);
    println!("{}", occupation);
    println!("{}", age);
    println!("{}", age == _age);

    age = 21;

    // errors
    // _age = 21; // immutable error - assign twice
    // age = 21.0; // initialised with i32

    println!("{}", age);
    println!("{}", age == _age);

    println!("{name}");
}
