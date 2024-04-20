use super::DistanceUnit;
use crate::units::Unit;
use quantity::LengthQuantity;
use std::ops::{Div, Mul};

#[derive(PartialEq, Debug, Copy, Clone, LengthQuantity)]
#[conversion(meter = 149.6e7, integer = true)] // 100 Pixel = 1 AU
pub struct Pixel {
    value: usize,
}

impl Pixel {
    pub const PER_ASTRO_UNIT: f32 = 100.;
}
