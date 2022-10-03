use std::io::{self, Write};

use git_demo::arithmetic as arith;

fn main() {
    let mut input = String::new();
    let mut user_name = String::new();

    print!("Enter your name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_name).unwrap();

    println!("Hello, {}!", user_name.trim());

    print!("Enter first number: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let x = input.trim().parse().unwrap_or(0);
    input.clear();

    print!("Enter second number: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let y = input.trim().parse().unwrap_or(0);

    println!("{x} + {y} = {}", arith::add(x, y));
}
