use std::io;

fn main() {
    println!("welcome to my Calculator");

    let mut input = String::new();
    println!("Enter the first number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let num1: f64 = input.trim().parse().expect("Please enter a valid number");

    input.clear();
    println!("Enter the operator (+, -, *, /):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let operator: char = input
        .trim()
        .chars()
        .next()
        .expect("Please enter a valid operator");
    input.clear();
    println!("Enter the second number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let num2: f64 = input.trim().parse().expect("Please enter a valid number");
    let result = match operator {
        '+' => num1 + num2,
        '-' => num1 - num2, 
        '*' => num1 * num2,
        '/' => num1 / num2,
        _ => {
            println!("wron operator.");
            return;
        }
    };

    println!("Answer: {}", result);
}
