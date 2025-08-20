// Lab 2: Static Allocation

// Create a few static variables.

static STATIC_A: u32 = 3;
static STATIC_B: i32 = -1000000;
static PI: f64 = 3.14159;

// This function also accesses and prints the static variables.
fn print_addresses_from_another_function() {
    println!("From another function:");
    // Note that the addresses printed here will be identical to those in main.
    println!(
        "  Static A: {}, Address: {:p}",
        STATIC_A, &STATIC_A
    );
    println!(
        "  Static B: {}, Address: {:p}",
        STATIC_B, &STATIC_B
    );
     println!(
        "  Pi      : {}, Address: {:p}",
        PI, &PI
    );
}


fn main() {
    // Print the values and memory addresses from the main function.
    // The addresses are fixed at compile time and do not change.
    println!(
        "Static A: {}, Address: {:p}",
        STATIC_A, &STATIC_A
    );
    println!(
        "Static B: {}, Address: {:p}",
        STATIC_B, &STATIC_B
    );
    println!(
        "Pi      : {}, Address: {:p}",
        PI, &PI
    );

    println!("\n--------------------------------------------------\n");

    // Call the other function to show the addresses are the same.
    print_addresses_from_another_function();

    println!("\n--------------------------------------------------\n");

    // --- Attempting to change a static variable ---
    // The following line is commented out because it will cause a compile-time error.
    // Rust enforces that static variables are immutable by default for thread safety.
    // STATIC_A = 5; // Error: cannot modify a static item

}
