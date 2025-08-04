use std::io;

// A tuple for employee data: (Name, Age, Salary)
type Employee = (String, u32, f64);

fn main() {
    // Get Salaries and Store in an Array ---
    let mut salaries: [f64; 5] = [0.0; 5];
    println!("--- Enter Employee Salaries ---");

    // Get salaries
    for i in 0..5 {
        loop {
            println!("Enter salary for employee {}: ", i + 1);

            let mut salary_input = String::new();
            io::stdin().read_line(&mut salary_input).expect("Failed to read line");

            match salary_input.trim().parse::<f64>() {
                Ok(num) => {
                    salaries[i] = num;
                    break;
                }
                Err(_) => {
                    println!("Invalid input. Please enter a valid number for the salary.");
                }
            }
        }
    }

    println!("\n--- Enter Employee Details ---");
    // Get Employee Details 
    let mut employees: [Employee; 5] = Default::default();

    for i in 0..5 {
        println!("Enter details for Employee {}:", i + 1);

        // --- Get Name ---
        println!("  Name: ");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        // Remove the newline character from the input.
        name = name.trim().to_string();

        // --- Get Age ---
        let age: u32 = loop {
            println!("  Age: ");
            let mut age_input = String::new();
            io::stdin().read_line(&mut age_input).expect("Failed to read line");
            
            // Loop until a valid unsigned 32-bit integer is entered.
            match age_input.trim().parse::<u32>() {
                Ok(num) => break num,
                Err(_) => println!("Invalid input. Please enter a valid number for age."),
            }
        };

        // The salary is taken from the `salaries` array we filled earlier.
        let salary = salaries[i];

        // Assign the new employee tuple to the correct index in the array.
        employees[i] = (name, age, salary);
    }

    // --- 3. Print All Employee Details ---
    println!("\n--- All Employee Details ---");
    for i in 0..5 {
        // Access the employee at the current index 'i'.
        println!(
            "Employee {}: Name = \"{}\", Age = {}, Salary = {:.2}",
            i + 1, employees[i].0, employees[i].1, employees[i].2
        );
    }

    // --- 4. Calculate and Print Statistics ---
    println!("\n--- Salary and Age Statistics ---");

    // Calculate Total and Average Salary
    let mut total_salary = 0.0;
    for salary in &salaries {
        total_salary += salary;
    }
    let average_salary = total_salary / 5 as f64;

    println!("Total Salary of all employees: {:.2}", total_salary);
    println!("Average Salary: {:.2}", average_salary);

    // Find Employee with the Highest Salary
    // The logic remains the same as it works on an iterator.
    let mut highest_salary_employee = &employees[0];
    for employee in &employees {
        if employee.2 > highest_salary_employee.2 {
            highest_salary_employee = employee;
        }
    }
    println!(
        "The employee with the highest salary is: {} with a salary of {:.2}",
        highest_salary_employee.0, highest_salary_employee.2
    );

    // Find the Oldest Employee
    let mut oldest_employee = &employees[0];
    for employee in &employees {
        if employee.1 > oldest_employee.1 {
            oldest_employee = employee;
        }
    }
    println!(
        "The oldest employee is: {}, {} years old",
        oldest_employee.0, oldest_employee.1
    );
}
