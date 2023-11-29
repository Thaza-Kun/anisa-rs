use std::ops::{Div, Mul, MulAssign};

pub mod length;
pub mod time;

pub trait Unit: Sized {
    fn new(value: f64) -> Self;
    fn unit() -> Self {
        Self::new(1f64)
    }
}
