use std::ops::{Div, Mul, MulAssign};

pub mod length;
pub mod time;

pub trait Unit: Sized + Copy {
    fn new(value: f32) -> Self;
    fn unit() -> Self {
        Self::new(1f32)
    }
}
