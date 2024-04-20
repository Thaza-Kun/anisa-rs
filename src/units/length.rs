use crate::units::Unit;
use astrounit::AstroUnit;
use meter::Meter;
use pixel::Pixel;
use std::ops::{Div, Mul};

pub mod astrounit;
pub mod meter;
pub mod pixel;
mod tests;

pub trait DistanceUnit: Mul<f32, Output = Self> + Div<f32, Output = Self> + Unit {
    const METER: f32;

    fn to<Other: DistanceUnit>() -> Other {
        Other::new(Self::METER / Other::METER)
    }

    fn kilometer(self) -> Meter
    where
        f32: Mul<Self, Output = f32>,
    {
        // Cheat way to convert to f32 (assymetric Mul: [Unit * f32 -> Unit] BUT [f32 * Unit -> f32])
        Meter::new(1. * (self * (Self::METER / 1_000.)))
    }

    fn meter(self) -> Meter
    where
        f32: Mul<Self, Output = f32>,
    {
        // Cheat way to convert to f32 (assymetric Mul: [Unit * f32 -> Unit] BUT [f32 * Unit -> f32])
        Meter::new(1. * (self * Self::METER))
    }

    fn centimeter(self) -> Meter
    where
        f32: Mul<Self, Output = f32>,
    {
        // Cheat way to convert to f32 (assymetric Mul: [Unit * f32 -> Unit] BUT [f32 * Unit -> f32])
        Meter::new(1. * (self * (Self::METER / 0.1)))
    }

    fn millimeter(self) -> Meter
    where
        f32: Mul<Self, Output = f32>,
    {
        // Cheat way to convert to f32 (assymetric Mul: [Unit * f32 -> Unit] BUT [f32 * Unit -> f32])
        Meter::new(1. * (self * (Self::METER / 0.01)))
    }
}
