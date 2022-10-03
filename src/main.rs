fn main() {
    let mut user_name = String::new();

    std::io::stdin().read_line(&mut user_name).unwrap();

    println!("Hello, {user_name}!");
}
