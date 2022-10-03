use std::io::{self, Write};

use git_demo::{arithmetic as arith, input};

fn main() {
    print!("Enter your name: ");
    io::stdout().flush().unwrap();

    println!("Hello, {}!", input::line());

    print!("Enter first number: ");
    io::stdout().flush().unwrap();
    let x: i32 = input::number();

    print!("Enter second number: ");
    io::stdout().flush().unwrap();
    let y: i32 = input::number();

    println!("{x} + {y} = {}", arith::add(x, y));
}
