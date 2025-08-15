use std::io;
use std::io::Write;


fn main() {
    let a = get_number_from_user("Enter the first number:");
    let b = get_number_from_user("Enter the second number:");
    let result = sum_two_numbers(a, b);
    println!("The sum of {} and {} is {}", a, b, result);
}


fn get_number_from_user(prompt: &str) -> i32{
    loop {
        print!("{prompt}");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read line");
            continue;
        }

        match input.trim().parse::<i32>() {
            Ok(n) => return n,
            Err(_) => eprintln!("Please type a number!"),
        }
        
    }
}

fn sum_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}