use std::ops::{Div, Mul};
use quantity::LengthQuantity;
use crate::units::Unit;
use super::DistanceUnit;

#[derive(LengthQuantity)]
#[conversion(meter=149.6e9)]
pub struct AstroUnit {
    value: f32
}