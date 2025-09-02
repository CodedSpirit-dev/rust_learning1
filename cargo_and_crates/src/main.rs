use std::io;
use std::io::Write;
use calculations::*;


fn main() {
    println!("🧮 Welcome to the Calculator!");

    loop {
        // Show menu
        println!("\nSelect an operation:");
        println!("1) Add two numbers");
        println!("2) Subtract two numbers");
        println!("3) Multiply two numbers");
        println!("4) Divide two numbers");
        println!("5) Quit");

        // Read user choice
        print!("Enter your choice (1 - 5): ");

        io::stdout().flush().unwrap(); // This line help to

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let choice: u8 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("\nInvalid input, enter a valid number");
            continue;
        }

        };

        // Handle choice

        match choice {
            1 => {
                let a = get_number_from_user("Enter the first number: ");
                let b = get_number_from_user("Enter the second number: ");
                let result = add(a.into(),b.into());
                println!("{} + {} = {}", a, b, result);
            }
            2 => {
                let a = get_number_from_user("Enter the first number: ");
                let b = get_number_from_user("Enter the second number: ");
                let result = multiply(a.into(),b.into());
                println!("{} X {} = {}", a, b, result);
            }
            3 => {
                let a = get_number_from_user("Enter the first number");
                let b = get_number_from_user("Enter the second number");
                let result = divide(a.into(),b.into());
                println!("{} / {} = {:?}", a, b, result);
            }
            4 => {
                let a = get_number_from_user("Enter the first number");
                let b = get_number_from_user("Enter the second number");
                let result = subtract(a.into(),b.into());
                println!("{} - {} = {}", a, b, result);
            }
            5 => {
                println!("Goodbye");
                break
            }
            _ => {
                println!("Invalid choice, pick 1-5");
            }
        }
    }

}


fn get_number_from_user(prompt: &str) -> f64{
    loop {
        print!("{prompt}");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read line");
            continue;
        }

        match input.trim().parse::<f64>() {
            Ok(n) => return n,
            Err(_) => eprintln!("Please type a number!"),
        }
        
    }
}
