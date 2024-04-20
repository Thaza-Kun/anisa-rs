use super::TemporalUnit;
use crate::units::Unit;
use quantity::TimeQuantity;
use std::ops::{Div, Mul};

#[derive(PartialEq, Debug, Copy, Clone, TimeQuantity)]
#[conversion(second = 1.)]
pub struct Second {
    value: f32,
}
