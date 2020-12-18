use crate::errors::ParcelError;

/// Weight of a parcel in kilograms.
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct Weight(f32);

impl Weight {
    pub fn new(weight: f32) -> Result<Self, ParcelError> {
        if weight <= 0.0 {
            Err(ParcelError::NonPositiveWeight(weight))
        } else {
            Ok(Self(weight))
        }
    }

    pub fn as_f32(self) -> f32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Weight;

    #[test]
    fn zero_weights_are_invalid() {
        assert!(Weight::new(0.0).is_err());
    }

    #[test]
    fn negative_weights_are_invalid() {
        assert!(Weight::new(-1.0).is_err());
    }
}
