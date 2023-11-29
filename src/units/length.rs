use std::ops::{Div, Mul};
use astrounit::AstroUnit;
use meter::Meter;
use pixel::Pixel;
use crate::units::Unit;

pub mod meter;
pub mod pixel;
pub mod astrounit;
mod tests;

pub trait DistanceUnit: Mul<f64, Output=f64> + Div<f64, Output=f64> + Unit{
    const METER: f64;

    fn to<Other: DistanceUnit>() -> Other {
        Other::new(Self::METER/Other::METER)
    }

    fn kilometer(self) -> Meter {
        Meter::new(self * (Self::METER / 1_000.))
    }

    fn meter(self) -> Meter {
       Meter::new(self * Self::METER)
    }

    fn centimeter(self) -> Meter {
        Meter::new(self * (Self::METER / 0.1))
    }


    fn millimeter(self) -> Meter {
        Meter::new(self * (Self::METER / 0.01))
    }
}
