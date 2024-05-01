use crate::units::Unit;
use std::ops::{Div, Mul};

pub trait VelocityUnit: Mul<f32, Output = Self> + Div<f32, Output = Self> + Unit {
    const MPS: f32;

    fn to<Other: TemporalUnit>() -> Other {
        Other::new(Self::MPS / Other::MPS)
    }

    fn meter_per_second(self) -> MeterPerSecond
    where
        f32: Mul<Self, Output = f32>,
    {
        MeterPerSecond::new(1. * (self * Self::MPS))
    }

    fn meter_per_hour(self) -> MeterPerSecond
    where
        f32: Mul<Self, Output = f32>,
    {
        MeterPerSecond::new(1. * ((self * Self::MPS) * SECONDS_PER_HOUR))
    }

    fn meter_per_day(self) -> MeterPerSecond
    where
        f32: Mul<Self, Output = f32>,
    {
        MeterPerSecond::new(1. * ((self * Self::MPS) * SECONDS_PER_DAY))
    }
}
