use std::io;

// Convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

// Convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// Show temperature with label using immutable reference
fn show_temperature(label: &str, value: &f64) {
    println!("{}: {:.2}", label, value);
}

// Adjust temperature by delta using mutable reference
fn adjust_temperature(value: &mut f64, delta: f64) {
    *value += delta;
}

fn main() {
    println!("Choose conversion direction:");
    println!("1) Celsius → Fahrenheit");
    println!("2) Fahrenheit → Celsius");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    println!("Enter the temperature value:");
    let mut temp_input = String::new();
    io::stdin().read_line(&mut temp_input).expect("Failed to read temperature");

    let mut temp: f64 = match temp_input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number format.");
            return;
        }
    };

    let result = match choice.trim() {
        "1" => celsius_to_fahrenheit(temp),
        "2" => fahrenheit_to_celsius(temp),
        _ => {
            println!("Invalid choice.");
            return;
        }
    };

    show_temperature("Converted temperature", &result);
    adjust_temperature(&mut temp, 0.5);
    show_temperature("Adjusted original input", &temp);
}
