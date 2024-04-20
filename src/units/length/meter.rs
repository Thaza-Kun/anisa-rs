use super::DistanceUnit;
use crate::units::Unit;
use quantity::LengthQuantity;
use std::ops::{Div, Mul};

#[derive(PartialEq, Debug, Copy, Clone, LengthQuantity)]
#[conversion(meter = 1.)]
pub struct Meter {
    value: f32,
}
