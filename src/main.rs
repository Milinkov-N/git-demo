use std::{
    io::{self, Write},
    process::exit,
};

use git_demo::{arithmetic as arith, input};

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

fn main() {
    print_flush("Enter your name: ");

    match input::line() {
        Ok(name) => println!("Hello, {name}!"),
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    }

    println!(
        "Choose operation:{}{}{}{}",
        "\n  1. Summation", "\n  2. Subdivision", "\n  3. Multiplication", "\n  4. Division"
    );

    print_flush("> ");
    let operation = input::number();

    if let Err(e) = operation {
        eprintln!("Error: {e}");
        exit(1);
    }

    match operation.unwrap() {
        1 => handle_operation(Op::Add),
        2 => handle_operation(Op::Sub),
        3 => handle_operation(Op::Mul),
        4 => handle_operation(Op::Div),
        op => eprintln!("Unknown operation \"{op}\""),
    }
}

fn print_flush(prompt: &str) {
    print!("{prompt}");

    if let Err(e) = io::stdout().flush() {
        eprintln!("Error: {e}");
        exit(1);
    }
}

fn numbers_input() -> Result<(i32, i32), input::InvalidInput> {
    print_flush("Enter first number: ");
    let x: i32 = input::number()?;

    print_flush("Enter second number: ");
    let y: i32 = input::number()?;

    Ok((x, y))
}

fn handle_operation(operation: Op) {
    match numbers_input() {
        Ok((x, y)) => {
            let result = match operation {
                Op::Add => arith::add(x, y),
                Op::Sub => arith::sub(x, y),
                Op::Mul => arith::mul(x, y),
                Op::Div => arith::div(x, y),
            };

            println!("{x} * {y} = {result}");
        }
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    }
}
