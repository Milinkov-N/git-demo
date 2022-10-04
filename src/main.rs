use std::io::{self, Write};

use git_demo::{arithmetic as arith, input};

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

fn main() {
    print!("Enter your name: ");
    io::stdout().flush().unwrap();

    println!("Hello, {}!", input::line().unwrap());

    println!(
        "Choose operation:{}{}{}{}",
        "\n  1. Summation", "\n  2. Subdivision", "\n  3. Multiplication", "\n  4. Division"
    );

    print!("> ");
    io::stdout().flush().unwrap();
    let operation: i32 = input::number().unwrap();

    match operation {
        1 => handle_operation(Op::Add),
        2 => handle_operation(Op::Sub),
        3 => handle_operation(Op::Mul),
        4 => handle_operation(Op::Div),
        op => unreachable!("Unknown operation \"{op}\""),
    }
}

fn numbers_input() -> (i32, i32) {
    print!("Enter first number: ");
    io::stdout().flush().unwrap();
    let x: i32 = input::number().unwrap();

    print!("Enter second number: ");
    io::stdout().flush().unwrap();
    let y: i32 = input::number().unwrap();

    (x, y)
}

fn handle_operation(operation: Op) {
    let (x, y) = numbers_input();

    let result = match operation {
        Op::Add => arith::add(x, y),
        Op::Sub => arith::sub(x, y),
        Op::Mul => arith::mul(x, y),
        Op::Div => arith::div(x, y),
    };

    println!("{x} * {y} = {result}");
}
