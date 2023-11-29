use quantity::TimeQuantity;
use crate::units::Unit;

#[derive(TimeQuantity)]
#[conversion(second=1.)]
pub struct Second {
    value: f64
}