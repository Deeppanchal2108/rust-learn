use std::env;
fn main() {
    println!("--------------------------------------------------------------");
    println!("Welcome to the Mini CLI Program!");
    let args: Vec<String> = env::args().collect();
    println!(" Here is  ur args{:?}", args);


    if args.len()<2{
        println!("Please provide a command line argument.");
        return;
    } 

    let command= &args[1];

    match command.as_str(){
        "greet"=>println!("Hello {}",&args[2]),
        "add"=>{
            println!("Adding numbers");
            let num1: i32 = args[2].parse().expect("Please provide a valid number");
            let num2: i32 = args[3].parse().expect("Please provide a valid number");
            let sum = num1 + num2;
            println!("The sum of {} and {} is {}", num1, num2, sum);

        },
        "mul"=>{
            println!("Multiplying numbers");
            let num1: i32 = args[2].parse().expect("Please provide a valid number");
            let num2: i32 = args[3].parse().expect("Please provide a valid number");
            let product = num1 * num2;
            println!("The product of {} and {} is {}", num1, num2, product);

        },
        "div"=>{
            println!("Dividing numbers");
            let num1: i32 = args[2].parse().expect("Please provide a valid number");
            let num2: i32 = args[3].parse().expect("Please provide a valid number");
            let div = num1 / num2;
            println!("The division of {} and {} is {}", num1, num2, div);

        },
        "sub"=>{
            println!("Subtracting numbers");
            let num1: i32 = args[2].parse().expect("Please provide a valid number");
            let num2: i32 = args[3].parse().expect("Please provide a valid number");
            let sub = num1 - num2;
            println!("The subtraction of {} and {} is {}", num1, num2, sub);

        },
        _=>println!("Unknown command: {}", command),
    }
}
