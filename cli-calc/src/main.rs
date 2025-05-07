use std::io::{self, Write};

fn main() {
    loop{
println!("Enter the mathematical equation eg (12*5) or quit");
io::stdout().flush().expect("Flush failed");

let mut input = String::new();
io::stdin().read_line(&mut input).expect("Something went wrong while taking input");

print!("Here is the input : {}",input);


    }
}
