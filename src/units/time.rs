use std::ops::{Div, Mul};
use second::Second;
use crate::units::Unit;

mod second;

pub trait TemporalUnit: Mul<f64, Output=f64> + Div<f64, Output=f64> + Div<Self, Output=f64> + Unit {
    const SECOND: f64;

    fn to<Other: TemporalUnit>() -> Other { Other::new(Self::SECOND/Other::SECOND)}

    fn second(self) -> Second { Second::new(self * Self::SECOND)}

    fn minute(self) -> Second { Second::new((self * Self::SECOND) / 60.)}

    fn hour(self) -> Second { Second::new(self.minute() / 60.) }

    fn day(self) -> Second { Second::new(self.hour() / 24. ) }
    fn week(self) -> Second { Second::new(self.day() / 7. ) }
}
