use crate::errors::ParcelError;

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct Dimensions(f32, f32, f32);

impl Dimensions {
    pub fn new(length: f32, width: f32, height: f32) -> Result<Self, ParcelError> {
        if length <= 0.0 || width <= 0.0 || height <= 0.0 {
            Err(ParcelError::NonPositiveDimensions(length, width, height))
        } else {
            Ok(Self(length, width, height))
        }
    }

    pub(crate) fn any(&self, f: fn(&f32) -> bool) -> bool {
        [self.0, self.1, self.2].iter().any(f)
    }

    pub(crate) fn all(&self, f: fn(&f32) -> bool) -> bool {
        [self.0, self.1, self.2].iter().all(f)
    }
}

#[cfg(test)]
mod tests {
    use super::Dimensions;
    #[test]
    fn zero_dimensions_are_invalid() {
        assert!(Dimensions::new(0.0, 0.0, 0.0).is_err());
    }

    #[test]
    fn negative_dimensions_are_invalid() {
        assert!(Dimensions::new(-1.0, -2.0, -3.14).is_err());
    }
}
