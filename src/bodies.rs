use crate::quantities::dynamics::Force;
use crate::quantities::spatial::{Cartesian, Quantity, Velocity};
use crate::quantities::Tensor;
use nalgebra::{DMatrix, DVector};
use std::collections::VecDeque;
use crate::Planet;
use crate::units::length::pixel::Pixel;
use crate::units::length::DistanceUnit;
use crate::units::length::meter::Meter;

#[derive(Default, Debug, Clone)]
pub struct AstroBody<X, const R: usize, const C: usize>
where
    X: Tensor<R, C> + Quantity,
{
    pub pos: X,
    pub velocity: Velocity<X, R, C>,
    pub force: Force<X, R, C>,
    pub mass: f32,
    pub radius: f32,
    pub color: Option<String>
}

impl From<&Planet> for AstroBody<Cartesian<f32, 2, Meter>, 2, 1> where {
    fn from(value: &Planet) -> Self {
        AstroBody::new_static(value.mass, Cartesian::zero()).set_color(value.color.clone())
    }
}

impl AstroBody<Cartesian<f32, 2, Meter>, 2, 1> {

    pub fn set_color(mut self, color: String) -> Self {
        self.color = Some(color);
        self
    }
    pub fn new_static(mass: f32, pos: Cartesian<f32, 2, Pixel>) -> Self {
        AstroBody {
            pos: Cartesian::from(pos),
            mass,
            radius: mass.log10(),
            ..Default::default()
        }
    }
    pub fn new_dynamic(
        mass: f32,
        pos: Cartesian<f32, 2, Meter>,
        velocity: Velocity<Cartesian<f32, 2, Meter>, 2, 1>,
    ) -> Self {
        AstroBody {
            pos,
            mass,
            radius: mass * 0.5,
            velocity,
            ..Default::default()
        }
    }

    /// Evaluates gravitational pull towards other AstroBody's.
    ///
    /// The function evaluates the Matrix:
    ///
    ///     f32 = G * M \[ΔR\] \[M_others\]
    ///
    /// where:
    /// - G         = `grav`: the gravitational constant
    /// - M         = `self.mass`: mass of self
    /// - ΔR        = unit displacement, as dim X n matrix:
    ///     ```matrix
    ///       Δr        |` Δx_1 ... Δx_n  `|
    ///     ------- =   |                  |
    ///     | r |^3     |_ Δy_1 ... Δy_n  _|
    ///     ```
    /// - M_others  = mass of others, as n x 1 column vector:
    ///     ```matrix
    ///          |` m_1 `|
    ///     M =  |  ...  |
    ///          |_ m_n _|
    ///     ```
    ///
    /// It is safe to include reference to self in others as self-interactions are nullified.
    pub fn gravitate(
        &mut self,
        others: &Vec<AstroBody<Cartesian<f32, 2, Meter>, 2, 1>>,
        grav: &f32,
    ) -> Force<Cartesian<f32, 2, Meter>, 2, 1> {
        //      |` m_1 `|
        // M =  |  ...  |
        //      |_ m_n _|
        let masses = DVector::<f32>::from_iterator(others.len(), others.iter().map(|x| x.mass));
        let coords = others.iter().fold(Vec::<f32>::new(), |mut a, b| {
            let data = (b.pos - self.pos).normed_values();
            for mut i in data {
                // If displacement is zero (interacts with self), i is Nan so we set it to 0.
                if i.is_nan() {
                    i = 0.
                }
                a.push(i)
            }
            a
        });
        //   Δr        |` Δx_1 ... Δx_n  `|
        // ------- =   |                  |
        // | r |^3     |_ Δy_1 ... Δy_n  _|
        let matrix = DMatrix::<f32>::from_vec(2, others.len(), coords);
        let f = (&matrix * masses).scale(*grav * self.mass);
        self.force = Force::new(f[0], f[1]);
        self.force
    }
    pub fn update(&mut self, step_time: f32) -> &Self {
        self.velocity +=
            Velocity::from_acceleration(&self.force.acceleration_of(&self.mass), &step_time);
        self.pos += Cartesian::from_velocity(&self.velocity, step_time);
        self.force = Force::zero();
        self
    }
    pub fn get_shift_from_origin(&self) -> Cartesian<f32, 2, Meter> {
        Cartesian::zero() - self.pos
    }
    pub fn get_velocity_shift_from_origin(&self) -> Velocity<Cartesian<f32, 2, Meter>, 2, 1> {
        Velocity::<Cartesian<f32, 2, Meter>, 2, 1>::zero() - self.velocity
    }
    pub fn shift_by(&mut self, &pos: &Cartesian<f32, 2, Meter>, &vel: &Velocity<Cartesian<f32, 2, Meter>, 2, 1>) {
        self.pos += pos;
        self.velocity += vel;
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use nalgebra::SimdComplexField;
    use crate::units::length::meter::Meter;

    /// Ensures that the gravity calculation is correct:
    /// - F_12 = -F21
    /// - F_1\[2+3\] + F_2\[1+3\] + F_3\[1+2\] = 0
    #[test]
    fn test_gravity() {
        let mut host = AstroBody::new_static(10., Cartesian::new(-1., -1.));
        let mut planet = AstroBody::new_static(0.4, Cartesian::new(1., 1.));

        let mut lists = vec![host.clone(), planet.clone()];

        assert_eq!(
            host.gravitate(&lists, &0.1),
            -planet.gravitate(&lists, &0.1)
        );

        let mut planet2 = AstroBody::new_static(0.4, Cartesian::new(0., 5.));
        lists.push(planet2.clone());

        assert_eq!(
            (host.gravitate(&lists, &0.1)
                + planet.gravitate(&lists, &0.1)
                + planet2.gravitate(&lists, &0.1))
            .covariant()
            .magnitude()
            .simd_round(),
            Force::<Cartesian<f32, 2, Meter>, 2, 1>::new(0., 0.)
                .covariant()
                .magnitude()
        )
    }
}

#[derive(Debug)]
pub struct Tracer<X, const R: usize, const C: usize>
where
    X: Tensor<R, C>,
{
    pub pos: VecDeque<X>,
}

impl<X> Default for Tracer<X, 2, 1>
where
    X: Tensor<2, 1>,
{
    fn default() -> Self {
        Tracer {
            pos: VecDeque::with_capacity(10),
        }
    }
}
