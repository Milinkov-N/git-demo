pub fn line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn number<T>() -> T
where
    T: std::str::FromStr + Default,
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    number_inner(&input)
}

fn number_inner<T>(input: &str) -> T
where
    T: std::str::FromStr + Default,
{
    input.trim().parse::<T>().unwrap_or(Default::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_number_basic() {
        assert_eq!(4u32, number_inner::<u32>("4"));
        assert_eq!(100usize, number_inner::<usize>("100"));
        assert_eq!(-3562i32, number_inner::<i32>("-3562"));
    }
}
