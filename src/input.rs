use crate::Number;

pub fn line() -> Result<String, InvalidInput> {
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .map_err(|_| InvalidInput(input.to_owned()))?;

    Ok(input.trim().to_owned())
}

pub fn number<T>() -> Result<T, InvalidInput>
where
    T: std::str::FromStr + Number + Default,
{
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .map_err(|_| InvalidInput(input.to_owned()))?;

    number_inner(&input)
}

fn number_inner<T>(input: &str) -> Result<T, InvalidInput>
where
    T: std::str::FromStr + Number + Default,
{
    input
        .trim()
        .parse::<T>()
        .map_err(|_| InvalidInput(input.to_owned()))
}

#[derive(Debug, PartialEq)]
pub struct InvalidInput(String);

impl std::fmt::Display for InvalidInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid input \"{}\"", self.0)
    }
}

impl std::error::Error for InvalidInput {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_number_basic() {
        assert_eq!(Ok(4u32), number_inner::<u32>("4"));
        assert_eq!(Ok(100usize), number_inner::<usize>("100"));
        assert_eq!(Ok(-3562i32), number_inner::<i32>("-3562"));
        assert!(number_inner::<u8>("562").is_err());
    }
}
