use std::ops::Add;

pub fn number<T>() -> T
where
    T: std::str::FromStr + Default,
{
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().parse::<T>().unwrap_or(Default::default())
}
