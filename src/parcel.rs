use crate::dimensions::Dimensions;

#[derive(Debug, PartialOrd, PartialEq)]
#[non_exhaustive]
pub struct Parcel {
    pub parcel_type: ParcelType,
    pub dimensions: Dimensions,
}

impl Parcel {
    pub fn new(dimensions: Dimensions) -> Self {
        let parcel_type = match dimensions {
            _ if dimensions.all(|dimension| dimension < &10.0) => ParcelType::Small,
            _ if dimensions.all(|dimension| dimension < &50.0) => ParcelType::Medium,
            _ if dimensions.all(|dimension| dimension < &100.0) => ParcelType::Large,
            _ if dimensions.any(|dimension| dimension >= &100.0) => ParcelType::XL,
            // We can assume no parcels have a dimension less than 0.0 because
            // they are checked when created
            _ => unreachable!(),
        };

        Self {
            parcel_type,
            dimensions,
        }
    }

    pub fn cost(&self) -> i32 {
        match self.parcel_type {
            ParcelType::Small => 300,
            ParcelType::Medium => 800,
            ParcelType::Large => 1500,
            ParcelType::XL => 2500,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
#[non_exhaustive]
pub enum ParcelType {
    Small,
    Medium,
    Large,
    XL,
}

#[cfg(test)]
mod tests {
    use super::{Parcel, ParcelType};
    use crate::dimensions::Dimensions;

    #[test]
    fn a_small_parcel_costs_3_dollars() {
        let parcel = Parcel::new(Dimensions::new(1.0, 3.0, 8.0).unwrap());
        assert_eq!(parcel.parcel_type, ParcelType::Small);
        assert_eq!(parcel.cost(), 300);
    }

    #[test]
    fn a_medium_parcel_costs_8_dollars() {
        let parcel = Parcel::new(Dimensions::new(4.0, 10.0, 3.0).unwrap());
        assert_eq!(parcel.parcel_type, ParcelType::Medium);
        assert_eq!(parcel.cost(), 800);

        let parcel = Parcel::new(Dimensions::new(4.0, 3.0, 26.0).unwrap());
        assert_eq!(parcel.parcel_type, ParcelType::Medium);
        assert_eq!(parcel.cost(), 800);
    }

    #[test]
    fn a_large_parcel_costs_15_dollars() {
        let parcel = Parcel::new(Dimensions::new(4.0, 10.0, 50.0).unwrap());
        assert_eq!(parcel.parcel_type, ParcelType::Large);
        assert_eq!(parcel.cost(), 1500);

        let parcel = Parcel::new(Dimensions::new(4.0, 3.0, 99.9).unwrap());
        assert_eq!(parcel.parcel_type, ParcelType::Large);
        assert_eq!(parcel.cost(), 1500);
    }

    #[test]
    fn an_xl_parcel_costs_25_dollars() {
        let parcel = Parcel::new(Dimensions::new(4.0, 10.0, 100.0).unwrap());
        assert_eq!(parcel.parcel_type, ParcelType::XL);
        assert_eq!(parcel.cost(), 2500);

        let parcel = Parcel::new(Dimensions::new(4.0, 314.15, 100.0).unwrap());
        assert_eq!(parcel.parcel_type, ParcelType::XL);
        assert_eq!(parcel.cost(), 2500);
    }
}
