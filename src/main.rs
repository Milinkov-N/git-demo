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
            print!("Enter first number: ");
            io::stdout().flush().unwrap();
            let x: i32 = input::number();

            print!("Enter second number: ");
            io::stdout().flush().unwrap();
            let y: i32 = input::number();

            println!("{x} + {y} = {}", arith::add(x, y));
        }
        2 => todo!(),
        3 => todo!(),
        4 => todo!(),
        op => unreachable!("Unknown operation \"{op}\""),
    }
}
