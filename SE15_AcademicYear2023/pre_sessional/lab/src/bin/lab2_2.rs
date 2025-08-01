use std::io;

fn main() {
    // Define a vector to store employee details
    let mut employees: Vec<(String, u32, u32)> = Vec::new();

    // Input details for 5 employees
    for i in 1..=5 {
        println!("Enter details for Employee {}:", i);

        // Read employee name
        let mut name = String::new();
        println!("Name: ");
        io::stdin().read_line(&mut name).expect("Failed to read input");
        let name = name.trim().to_string();

        // Read employee age
        let mut age = String::new();
        println!("Age: ");
        io::stdin().read_line(&mut age).expect("Failed to read input");
        let age: u32 = age.trim().parse().expect("Please enter a valid number");

        // Read employee salary
        let mut salary = String::new();
        println!("Salary: ");
        io::stdin().read_line(&mut salary).expect("Failed to read input");
        let salary: u32 = salary.trim().parse().expect("Please enter a valid number");

        // Store the employee details as a tuple
        employees.push((name, age, salary));
    }

    // Print employee details
    println!("\nEmployee Details:");
    for (i, employee) in employees.iter().enumerate() {
        println!(
            "Employee {}: Name = \"{}\", Age = {}, Salary = {}",
            i + 1,
            employee.0,
            employee.1,
            employee.2
        );
    }

    // Find the employee with the highest salary
    let highest_salary_employee = employees.iter().max_by_key(|e| e.2).unwrap();
    
    // Find the oldest employee
    let oldest_employee = employees.iter().max_by_key(|e| e.1).unwrap();
    
    // Print highest salary and oldest employee details
    println!(
        "The employee with the highest salary is: {} with a salary of {}",
        highest_salary_employee.0, highest_salary_employee.2
    );
    println!(
        "The oldest employee is: {}, {} years old",
        oldest_employee.0, oldest_employee.1
    );
}