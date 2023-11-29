#[cfg(test)]
mod test {
    use length::DistanceUnit;
    use crate::units::length::astrounit::AstroUnit;
    use crate::units::length::pixel::Pixel;
    use crate::units::*;
    #[test]
    fn test_conversion_to_pixel() {
        let au = AstroUnit::new(1.);

        assert_eq!(AstroUnit::to::<Pixel>(), Pixel::new(100.))
    }
}
