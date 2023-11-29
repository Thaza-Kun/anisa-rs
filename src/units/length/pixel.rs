use std::ops::{Div, Mul};
use quantity::LengthQuantity;
use super::DistanceUnit;
use crate::units::Unit;

#[derive(PartialEq, Debug, LengthQuantity)]
#[conversion(meter=149.6e7, integer=true)] // 100 Pixel = 1 AU
pub struct Pixel {
    value: usize
}

impl Pixel {
    pub const PER_ASTRO_UNIT: f32 = 100.;
}