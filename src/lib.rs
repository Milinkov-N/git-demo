use std::ops::{Add, Div, Mul, Rem, Sub};

pub mod arithmetic;
pub mod input;

pub trait Number<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Self>
    + Div<Rhs, Output = Self>
    + Sub<Rhs, Output = Self>
    + Mul<Rhs, Output = Self>
    + Rem<Rhs, Output = Self>
    + Sized
{
}

impl<T, Rhs, Output> Number<Rhs, Output> for T where
    T: Add<Rhs, Output = Self>
        + Div<Rhs, Output = Self>
        + Sub<Rhs, Output = Self>
        + Mul<Rhs, Output = Self>
        + Rem<Rhs, Output = Self>
{
}
