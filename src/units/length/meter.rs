use std::ops::{Div, Mul};
use crate::units::Unit;
use quantity::LengthQuantity;
use super::DistanceUnit;

#[derive(PartialEq, Debug, Copy, Clone, LengthQuantity)]
#[conversion(meter=1.)]
pub struct Meter {
    value: f32
}