use std::io::{self, Write};

fn main() {
    // A vector to store the inventory. Each product is a tuple.
    let mut inventory: Vec<(u32, String, u32)> = Vec::new();

    // Add some initial data for demonstration purposes.
    inventory.push((101, String::from("Laptop"), 20));
    inventory.push((102, String::from("Mouse"), 150));
    inventory.push((103, String::from("Keyboard"), 75));

    // The main program loop continues until the user chooses to exit.
    loop {
        // ## Display Menu ##
        println!("\n--- Warehouse Inventory Management ---");
        println!("1. Add New Product");
        println!("2. Update Stock Quantity");
        println!("3. Remove Product");
        println!("4. List All Products");
        println!("5. Exit");
        println!("------------------------------------");
        print!("Enter your choice: ");

        // Ensure the prompt appears before reading input.
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        // ## Input Validation ##
        // Parse the user's choice, handling non-numeric input.
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\n[Error] Invalid input! Please enter a number.");
                continue; // Restart the loop.
            }
        };

        // ## Inventory Management ##
        match choice {
            // ### 1) Add a new product ###
            1 => {
                println!("\n-- Add New Product --");
                let id: u32;

                // Loop until a unique, valid ID is provided.
                loop {
                    print!("Enter Product ID: ");
                    io::stdout().flush().unwrap();
                    let mut id_input = String::new();
                    io::stdin().read_line(&mut id_input).expect("Failed to read line");

                    if let Ok(num) = id_input.trim().parse::<u32>() {
                        // Check if the ID already exists.
                        let mut id_exists = false;
                        for (existing_id, _, _) in &inventory {
                            if *existing_id == num {
                                id_exists = true;
                                break;
                            }
                        }

                        if id_exists {
                            println!("[Error] Product ID {} already exists.", num);
                        } else {
                            id = num;
                            break; // Exit the loop with a valid ID.
                        }
                    } else {
                        println!("[Error] Invalid ID. Please enter a positive number.");
                    }
                }

                // Get the product name.
                print!("Enter Product Name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim().to_string();

                // Get the product quantity.
                let quantity: u32;
                loop {
                    print!("Enter Quantity: ");
                    io::stdout().flush().unwrap();
                    let mut quantity_input = String::new();
                    io::stdin().read_line(&mut quantity_input).expect("Failed to read line");
                    if let Ok(num) = quantity_input.trim().parse() {
                        quantity = num;
                        break;
                    } else {
                        println!("[Error] Invalid quantity. Please enter a positive number.");
                    }
                }

                // Add the new product to the inventory.
                inventory.push((id, name, quantity));
                println!("\nProduct added successfully!");
            }

            // ### 2) Update the stock quantity ###
            2 => {
                println!("\n-- Update Stock Quantity --");
                print!("Enter Product ID to update: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Failed to read line");

                if let Ok(id_to_update) = id_input.trim().parse::<u32>() {
                    let mut product_found = false;
                    for product in &mut inventory {
                        // `product.0` is the ID in the tuple `(ID, Name, Quantity)`.
                        if product.0 == id_to_update {
                            println!("Found product: {}, Current Quantity: {}", product.1, product.2);
                            product_found = true;
                            
                            // Loop for new quantity input.
                            loop {
                                print!("Enter new quantity: ");
                                io::stdout().flush().unwrap();
                                let mut quantity_input = String::new();
                                io::stdin().read_line(&mut quantity_input).expect("Failed to read line");

                                if let Ok(new_quantity) = quantity_input.trim().parse::<u32>() {
                                    product.2 = new_quantity; // Update the quantity.
                                    println!("\nQuantity updated successfully!");
                                    break;
                                } else {
                                     println!("[Error] Invalid quantity. Please enter a positive number.");
                                }
                            }
                            break; // Exit the main search loop.
                        }
                    }
                    if !product_found {
                        println!("[Error] Product with ID {} not found.", id_to_update);
                    }
                } else {
                    println!("[Error] Invalid ID format.");
                }
            }
            
            // ### 3) Remove a product ###
            3 => {
                println!("\n-- Remove Product --");
                print!("Enter Product ID to remove: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Failed to read line");

                if let Ok(id_to_remove) = id_input.trim().parse::<u32>() {
                    let mut index_to_remove: Option<usize> = None;
                    // Find the index of the product to remove.
                    for (index, (id, _, _)) in inventory.iter().enumerate() {
                        if *id == id_to_remove {
                            index_to_remove = Some(index);
                            break;
                        }
                    }

                    // If an index was found, remove the item at that index.
                    if let Some(index) = index_to_remove {
                        inventory.remove(index);
                        println!("\nProduct with ID {} removed successfully.", id_to_remove);
                    } else {
                        println!("[Error] Product with ID {} not found.", id_to_remove);
                    }
                } else {
                    println!("[Error] Invalid ID format.");
                }
            }
            
            // ### 4) List all products ###
            4 => {
                println!("\n-- Current Inventory --");
                if inventory.is_empty() {
                    println!("The warehouse is empty.");
                } else {
                    for (id, name, quantity) in &inventory {
                        println!("{} {}5 {}", id, name, quantity);
                    }
                }
            }
            
            // ### 5) Exit program ###
            5 => {
                println!("Exiting program.");
                break; // Exit the main loop.
            }
            
            // Handle invalid menu numbers.
            _ => {
                println!("\n[Error] Invalid choice. Please enter a number between 1 and 5.");
            }
        }
    }
}