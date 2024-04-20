use crate::quantities::Tensor;
use crate::units::length::meter::Meter;
use crate::units::length::pixel::Pixel;
use crate::units::length::DistanceUnit;
use crate::units::Unit;
use nalgebra::{ArrayStorage, Const, Matrix, Rotation2, SVector, SimdComplexField, SimdRealField};
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, Neg, Sub},
};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Cartesian<F: SimdRealField, const R: usize, U: DistanceUnit> {
    elems: SVector<F, R>,
    unit: U,
}

impl<F: SimdRealField, const R: usize, U: DistanceUnit> Sub for Cartesian<F, R, U> {
    type Output = Cartesian<F, R, U>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            elems: self.elems - rhs.elems,
            unit: self.unit,
        }
    }
}

impl<const R: usize> From<Cartesian<f32, R, Meter>> for Cartesian<f32, R, Pixel> {
    fn from(value: Cartesian<f32, R, Meter>) -> Self {
        Self {
            elems: value.elems.map(|x| x / value.unit.meter()),
            unit: Pixel::unit(),
        }
    }
}

impl<const R: usize> From<Cartesian<f32, R, Pixel>> for Cartesian<f32, R, Meter> {
    fn from(value: Cartesian<f32, R, Pixel>) -> Self {
        let scale = value.unit.meter();
        Self {
            elems: value.elems.map(|x| x * scale),
            unit: Meter::unit(),
        }
        // todo!()
    }
}

impl<F: SimdRealField, const R: usize, U: DistanceUnit> Quantity for Cartesian<F, R, U> {
    type UNIT = U;
}

pub trait Quantity {
    type UNIT: Unit;
}

impl<U: DistanceUnit> Cartesian<f32, 2, U>
where
    Cartesian<f32, 2, U>: Sub<Cartesian<f32, 2, U>, Output = Cartesian<f32, 2, U>>,
{
    pub fn to<Other: DistanceUnit>(self) -> Cartesian<f32, 2, Other>
    where
        f32: Div<Other, Output = f32>,
    {
        let unit: Other = U::to::<Other>();
        let values = self.elems.map(|x| x / unit);
        Cartesian {
            elems: values,
            unit: Other::unit(),
        }
    }
    pub fn zero() -> Self {
        Cartesian::new(0., 0.)
    }
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            elems: SVector::<f32, 2>::new(x, y),
            unit: U::unit(),
        }
    }
    pub fn from_velocity(velocity: &Velocity<Cartesian<f32, 2, U>, 2, 1>, step_time: f32) -> Self {
        Cartesian {
            elems: velocity.elems.elems.scale(step_time),
            unit: U::unit(),
        }
    }

    pub fn horizontal(&self) -> f32 {
        self.covariant().x
    }

    pub fn vertical(&self) -> f32 {
        self.covariant().y
    }

    #[deprecated = "A Radial Coordinate system is planned"]
    pub fn with_magnitude(magnitude: f32, angle: f32, origin: Cartesian<f32, 2, U>) -> Self {
        origin - Cartesian::<f32, 2, U>::new(magnitude * angle.cos(), magnitude * angle.sin())
    }

    pub fn rotate_perpendicular_to(
        self,
        other: &Cartesian<f32, 2, U>,
        angle: f32,
    ) -> SVector<f32, 2> {
        ((other.elems - self.elems).transpose() * Rotation2::new(angle + 90.)).transpose()
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Velocity<X, const R: usize, const C: usize>
where
    X: Tensor<R, C> + Quantity,
{
    elems: X,
}

impl<X, const R: usize, const C: usize> Quantity for Velocity<X, R, C>
where
    X: Tensor<R, C> + Quantity,
{
    type UNIT = X::UNIT;
}

impl<X, const R: usize, const C: usize> Tensor<R, C> for Velocity<X, R, C>
where
    X: Tensor<R, C> + Quantity,
{
    type Value = X::Value;

    fn covariant(
        &self,
    ) -> &Matrix<Self::Value, Const<R>, Const<C>, ArrayStorage<Self::Value, R, C>> {
        self.elems.covariant()
    }
}

impl Velocity<Cartesian<f32, 2, Meter>, 2, 1> {
    pub fn zero() -> Self {
        Velocity {
            elems: Cartesian::zero(),
        }
    }

    pub fn horizontal(&self) -> f32 {
        self.covariant().x
    }

    pub fn vertical(&self) -> f32 {
        self.covariant().y
    }
    pub fn new(x: f32, y: f32) -> Self {
        Velocity {
            elems: Cartesian::new(x, y),
        }
    }

    pub fn new_perpendicular_to(
        magnitude: f32,
        here: &Cartesian<f32, 2, Meter>,
        other: &Cartesian<f32, 2, Meter>,
        angle: f32,
    ) -> Velocity<Cartesian<f32, 2, Meter>, 2, 1> {
        let dir = here.rotate_perpendicular_to(other, angle);
        Velocity {
            elems: Cartesian {
                elems: dir.normalize().scale(magnitude),
                unit: Meter::unit(),
            },
        }
    }

    pub fn from_acceleration(
        accel: &Acceleration<Cartesian<f32, 2, Meter>, 2, 1>,
        step: &f32,
    ) -> Self {
        Velocity {
            elems: Cartesian {
                elems: accel.covariant().scale(*step),
                unit: Meter::unit(),
            },
        }
    }
}

pub struct Acceleration<X, const R: usize, const C: usize>
where
    X: Tensor<R, C>,
{
    elems: X,
}

impl Acceleration<Cartesian<f32, 2, Meter>, 2, 1> {
    pub fn new(x: f32, y: f32) -> Self {
        Acceleration {
            elems: Cartesian::new(x, y),
        }
    }
}

// --- Custom Trait Impls ---
impl<const R: usize, U: DistanceUnit> Tensor<R, 1> for Cartesian<f32, R, U> {
    type Value = f32;
    fn covariant(
        &self,
    ) -> &Matrix<Self::Value, Const<R>, Const<1>, ArrayStorage<Self::Value, R, 1>> {
        &self.elems
    }
}

impl<X, const R: usize, const C: usize> Tensor<R, C> for Acceleration<X, R, C>
where
    X: Tensor<R, C> + Quantity,
{
    type Value = X::Value;

    fn covariant(
        &self,
    ) -> &Matrix<Self::Value, Const<R>, Const<C>, ArrayStorage<Self::Value, R, C>> {
        &self.elems.covariant()
    }
}

// --- Trivial Trait Impls --

impl<U: DistanceUnit> Default for Cartesian<f32, 2, U> {
    fn default() -> Self {
        Cartesian::zero()
    }
}

impl<const R: usize, U: DistanceUnit> Neg for Cartesian<f32, R, U> {
    type Output = Cartesian<f32, R, U>;

    fn neg(self) -> Self::Output {
        Cartesian {
            elems: self.elems.scale(-1.),
            unit: U::unit(),
        }
    }
}

impl<F: SimdRealField, const R: usize, U: DistanceUnit> AddAssign for Cartesian<F, R, U> {
    fn add_assign(&mut self, rhs: Self) {
        self.elems += rhs.elems
    }
}

impl<F: SimdRealField, const R: usize, U: DistanceUnit> Add for Cartesian<F, R, U> {
    type Output = Cartesian<F, R, U>;

    fn add(self, rhs: Self) -> Self::Output {
        Cartesian {
            elems: self.elems + rhs.elems,
            unit: U::unit(),
        }
    }
}

impl<X, const R: usize, const C: usize> Sub for Velocity<X, R, C>
where
    X: Tensor<R, C> + Quantity + Sub,
    <X as Sub>::Output: Tensor<R, C> + Quantity,
{
    type Output = Velocity<<X as Sub>::Output, R, C>;

    fn sub(self, rhs: Self) -> Self::Output {
        Velocity {
            elems: self.elems - rhs.elems,
        }
    }
}

impl<X, const R: usize, const C: usize> Neg for Velocity<X, R, C>
where
    X: Tensor<R, C> + Quantity + Neg,
    <X as Neg>::Output: Tensor<R, C> + Quantity,
{
    type Output = Velocity<<X as Neg>::Output, R, C>;

    fn neg(self) -> Self::Output {
        Velocity { elems: -self.elems }
    }
}

impl<X, const R: usize, const C: usize> AddAssign for Velocity<X, R, C>
where
    X: Tensor<R, C> + Quantity + AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.elems += rhs.elems
    }
}

impl<X, const R: usize, const C: usize> Add for Velocity<X, R, C>
where
    X: Tensor<R, C> + Quantity + Add,
    X::Output: Tensor<R, C> + Quantity,
{
    type Output = Velocity<X::Output, R, C>;

    fn add(self, rhs: Self) -> Self::Output
    where
        <X as Add>::Output: Tensor<R, C>,
    {
        Velocity {
            elems: self.elems + rhs.elems,
        }
    }
}
