use crate::quantities::spatial::{Acceleration, Cartesian};
use crate::quantities::Tensor;
use nalgebra::{ArrayStorage, ComplexField, Const, Matrix};
use std::ops::{Add, AddAssign, Neg};
use crate::units::Unit;
use crate::units::length::meter::Meter;

#[derive(Default, PartialEq, Debug, Copy, Clone)]
pub struct Force<X, const R: usize, const C: usize>
where
    X: Tensor<R, C>,
{
    elems: X,
}

impl<X, const R: usize, const C: usize> Tensor<R, C> for Force<X, R, C>
where
    X: Tensor<R, C>,
{
    type Value = X::Value;

    fn covariant(
        &self,
    ) -> &Matrix<Self::Value, Const<R>, Const<C>, ArrayStorage<Self::Value, R, C>> {
        &self.elems.covariant()
    }
}

impl<X, const R: usize, const C: usize> Neg for Force<X, R, C>
where
    X: Tensor<R, C> + Neg,
    X::Output: Tensor<R, C>,
{
    type Output = Force<X::Output, R, C>;

    fn neg(self) -> Self::Output {
        Force { elems: -self.elems }
    }
}

impl<X, const R: usize, const C: usize> AddAssign for Force<X, R, C>
where
    X: Tensor<R, C> + AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.elems += rhs.elems
    }
}

impl<X, const R: usize, const C: usize> Add for Force<X, R, C>
where
    X: Tensor<R, C> + Add,
    <X as Add>::Output: Tensor<R, C>,
{
    type Output = Force<X::Output, R, C>;

    fn add(self, rhs: Self) -> Self::Output
    where
        <X as Add>::Output: Tensor<R, C>,
    {
        Force {
            elems: self.elems + rhs.elems,
        }
    }
}

impl Force<Cartesian<f32, 2, Meter>, 2, 1> {
    pub fn zero() -> Self {
        Force {
            elems: Cartesian::zero(),
        }
    }
    pub fn new(x: f32, y: f32) -> Self {
        Force {
            elems: Cartesian::new(x, y),
        }
    }

    pub fn horizontal(&self) -> f32 {
        self.elems.horizontal()
    }
    pub fn vertical(&self) -> f32 {
        self.elems.vertical()
    }

    pub fn acceleration_of(&self, mass: &f32) -> Acceleration<Cartesian<f32, 2, Meter>, 2, 1> {
        Acceleration::new(
            self.horizontal().scale(1. / mass),
            self.vertical().scale(1. / mass),
        )
    }
}
