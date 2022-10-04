use crate::Number;

pub fn add<T>(x: T, y: T) -> T
where
    T: Number,
{
    x + y
}

pub fn sub<T>(x: T, y: T) -> T
where
    T: Number,
{
    x - y
}

pub fn mul<T>(x: T, y: T) -> T
where
    T: Number,
{
    x * y
}

pub fn div<T>(x: T, y: T) -> T
where
    T: Number,
{
    x / y
}
