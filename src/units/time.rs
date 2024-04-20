use crate::units::Unit;
use second::Second;
use std::ops::{Div, Mul};

mod second;

#[allow(unused)]
const SECONDS_PER_MINUTE: f32 = 60.;
#[allow(unused)]
const SECONDS_PER_HOUR: f32 = 60. * SECONDS_PER_MINUTE;
#[allow(unused)]
const SECONDS_PER_DAY: f32 = 24. * SECONDS_PER_HOUR;
#[allow(unused)]
const SECONDS_PER_WEEK: f32 = 7. * SECONDS_PER_DAY;
#[allow(unused)]
const SECONDS_PER_MONTH: f32 = 4. * SECONDS_PER_WEEK;
#[allow(unused)]
const SECONDS_PER_YEAR: f32 = 12. * SECONDS_PER_MONTH;

pub trait TemporalUnit: Mul<f32, Output = Self> + Div<f32, Output = Self> + Unit {
    const SECOND: f32;

    fn to<Other: TemporalUnit>() -> Other {
        Other::new(Self::SECOND / Other::SECOND)
    }

    fn second(self) -> Second
    where
        f32: Mul<Self, Output = f32>,
    {
        Second::new(1. * (self * Self::SECOND))
    }

    fn minute(self) -> Second
    where
        f32: Mul<Self, Output = f32>,
    {
        Second::new(1. * ((self * Self::SECOND) / SECONDS_PER_MINUTE))
    }

    fn hour(self) -> Second
    where
        f32: Mul<Self, Output = f32>,
    {
        Second::new(1. * ((self * Self::SECOND) / SECONDS_PER_HOUR))
    }

    fn day(self) -> Second
    where
        f32: Mul<Self, Output = f32> + Mul<f32, Output = f32>,
    {
        Second::new(1. * ((self * Self::SECOND) / SECONDS_PER_DAY))
    }
    fn week(self) -> Second
    where
        f32: Mul<Self, Output = f32> + Mul<f32, Output = f32>,
    {
        Second::new(1. * ((self * Self::SECOND) / SECONDS_PER_WEEK))
    }
    fn month(self) -> Second
    where
        f32: Mul<Self, Output = f32> + Mul<f32, Output = f32>,
    {
        Second::new(1. * ((self * Self::SECOND) / SECONDS_PER_MONTH))
    }
    fn year(self) -> Second
    where
        f32: Mul<Self, Output = f32> + Mul<f32, Output = f32>,
    {
        Second::new(1. * ((self * Self::SECOND) / SECONDS_PER_YEAR))
    }
}
