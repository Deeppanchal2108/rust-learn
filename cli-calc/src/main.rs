use std::io::{self, Write};

fn main() {
    loop {
        println!(" ════════════════════════════════");
        println!("✨ Welcome to the CLI Calculator ✨");
        println!("════════════════════════════════ ");


        println!("Enter the first number");
        io::stdout().flush().expect("Flush failed");

        let mut first = String::new();
        io::stdin()
            .read_line(&mut first)
            .expect("Something went wrong while taking input");
        let first=first.trim();

         println!("Enter the second number");
        io::stdout().flush().expect("Flush failed");

        let mut second = String::new();
        io::stdin()
            .read_line(&mut second)
            .expect("Something went wrong while taking input");
        let second=second.trim();


        println!("Enter the operation (+, -, *, /)");
        io::stdout().flush().expect("Flush failed");
        let mut operation = String::new();
        io::stdin()
            .read_line(&mut operation)
            .expect("Something went wrong while taking input");
        let operation=operation.trim();

        match operation{
            
            "+"=>{
                print!("Plus operation");
                let first: f64 = first.parse().expect("Invalid number");
                let second: f64 = second.parse().expect("Invalid number");
                let result = first + second;
                println!("The result of {} + {} is {}", first, second, result);
            },

            "-"=>{
                print!("Minus operation");
                 let first: f64 = first.parse().expect("Invalid number");
                let second: f64 = second.parse().expect("Invalid number");
                let result = first - second;
                println!("The result of {} - {} is {}", first, second, result);
            },  
            "*"=>{
                print!("Multiplication operation");
                 let first: f64 = first.parse().expect("Invalid number");
                let second: f64 = second.parse().expect("Invalid number");
                let result = first * second;
                println!("The result of {} * {} is {}", first, second, result);
            },
            "/"=>{
                print!("Division operation");
                 let first: f64 = first.parse().expect("Invalid number");
                let second: f64 = second.parse().expect("Invalid number");
                if second == 0.0 {
                    println!("Error: Division by zero is not allowed.");
                    continue;
                }
                let result = first / second;
                println!("The result of {} / {} is {}", first, second, result);
            },
            _ => {
                println!("Invalid operation");
                continue;
            }
        }
        

        println!("Do you want to continue? (y/n)");
        io::stdout().flush().expect("Flush failed");

        let mut continue_input = String::new();
        io::stdin()
            .read_line(&mut continue_input)
            .expect("Something went wrong while taking input");
        let continue_input = continue_input.trim();
    


        if continue_input.to_lowercase() != "y" {
            println!("Exiting the calculator. Goodbye!");
            break;
        }
    }
}
