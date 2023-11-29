use nalgebra::{
    ArrayStorage, ComplexField, Const, Dim, Dyn, Matrix, RawStorage, Scalar, SimdComplexField,
    SimdRealField, Storage,
};
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub mod dynamics;
pub mod spatial;

pub trait Tensor<const R: usize, const C: usize> {
    type Value: Scalar + ComplexField;
    fn covariant(
        &self,
    ) -> &Matrix<Self::Value, Const<R>, Const<C>, ArrayStorage<Self::Value, R, C>>;

    // fn contravariant(&self) -> &Matrix<Self::Value, Const<C>, Const<R>, ArrayStorage<Self::Value, C, R>> {
    //     &self.covariant().transpose()
    // }

    fn values(&self) -> Vec<Self::Value> {
        self.covariant().data.as_slice().clone().to_vec()
    }

    fn normed_values(&self) -> Vec<Self::Value> {
        self.covariant()
            .normalize()
            .data
            .as_slice()
            .clone()
            .to_vec()
    }
}

pub trait Deriveable {
    type Manipulated;
    type Responding;

    type Output: AntiDeriveable<Output=Self>;
}

pub trait AntiDeriveable {
    type Manipulated;
    type Responding;

    type Output: Deriveable<Output=Self>;

}
