use std::io::{self, Write};

use git_demo::{arithmetic as arith, input};

fn main() {
    print!("Enter your name: ");
    io::stdout().flush().unwrap();

    println!("Hello, {}!", input::line());

    println!(
        "Choose operation:{}{}{}{}",
        "\n  1. Summation", "\n  2. Subdivision", "\n  3. Multiplication", "\n  4. Division"
    );

    print!("> ");
    io::stdout().flush().unwrap();
    let operation: i32 = input::number();

    match operation {
        1 => {
            let (x, y) = numbers_input();

            println!("{x} + {y} = {}", arith::add(x, y));
        }
        2 => {
            let (x, y) = numbers_input();

            println!("{x} - {y} = {}", arith::sub(x, y));
        }
        3 => {
            let (x, y) = numbers_input();

            println!("{x} * {y} = {}", arith::mul(x, y));
        }
        4 => {
            let (x, y) = numbers_input();

            println!("{x} / {y} = {}", arith::div(x, y));
        }
        op => unreachable!("Unknown operation \"{op}\""),
    }
}

fn numbers_input() -> (i32, i32) {
    print!("Enter first number: ");
    io::stdout().flush().unwrap();
    let x: i32 = input::number();

    print!("Enter second number: ");
    io::stdout().flush().unwrap();
    let y: i32 = input::number();

    (x, y)
}
