use super::DistanceUnit;
use crate::units::Unit;
use quantity::LengthQuantity;
use std::ops::{Div, Mul};

#[derive(PartialEq, Debug, Copy, Clone, LengthQuantity)]
#[conversion(meter = 149.6e9)]
pub struct AstroUnit {
    value: f32,
}
