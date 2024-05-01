use super::TemporalUnit;
use crate::units::Unit;
use quantity::TimeQuantity;
use std::ops::{Div, Mul};

#[derive(PartialEq, Debug, Copy, Clone, TimeQuantity)]
#[conversion(second = 1./24.)]
pub struct Frame {
    value: f32,
}
