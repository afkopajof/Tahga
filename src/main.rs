use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter an integer:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let trimmed_input = input.trim();
    let num: i32 = trimmed_input.parse().expect("Invalid input");

    let result = num * trimmed_input.len() as i32;
    println!("Result: {}", result);
}
