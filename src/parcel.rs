use crate::dimensions::Dimensions;
use crate::weight::Weight;

#[derive(Debug, PartialOrd, PartialEq)]
#[non_exhaustive]
pub struct Parcel {
    pub parcel_type: ParcelType,
    pub dimensions: Dimensions,
    pub weight: Weight,
}

impl Parcel {
    pub fn new(dimensions: Dimensions, weight: Weight) -> Self {
        let parcel_type = match dimensions {
            _ if weight.as_f32() >= 50.0 => ParcelType::Heavy,
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
            weight,
        }
    }

    pub fn cost(&self) -> i32 {
        self.parcel_type_cost() + self.over_weight_limit_cost()
    }

    fn parcel_type_cost(&self) -> i32 {
        match self.parcel_type {
            ParcelType::Small => 300,
            ParcelType::Medium => 800,
            ParcelType::Large => 1500,
            ParcelType::XL => 2500,
            ParcelType::Heavy => 5000,
        }
    }

    pub fn over_weight_limit_cost(&self) -> i32 {
        if self.weight > self.weight_limit() {
            // I have taken the spec to mean "for every *whole* kilogram over the weight limit
            // an additional charge of $2 applies.
            // The `200` below can be parameterised when required.
            self.over_weight_limit_multiplier()
                * (self.weight.as_f32() - self.weight_limit().as_f32()).floor() as i32
        } else {
            0
        }
    }

    pub fn weight_limit(&self) -> Weight {
        Weight::new(match self.parcel_type {
            ParcelType::Small => 1.0,
            ParcelType::Medium => 3.0,
            ParcelType::Large => 6.0,
            ParcelType::XL => 10.0,
            ParcelType::Heavy => 50.0,
        })
        .unwrap()
    }

    pub fn over_weight_limit_multiplier(&self) -> i32 {
        match self.parcel_type {
            ParcelType::Heavy => 100,
            _ => 200,
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
    Heavy,
}

#[cfg(test)]
mod tests {
    use crate::dimensions::Dimensions;
    use crate::weight::Weight;

    use super::{Parcel, ParcelType};

    #[test]
    fn a_small_parcel_costs_3_dollars() {
        let parcel = Parcel::new(
            Dimensions::new(1.0, 3.0, 8.0).unwrap(),
            Weight::new(0.5).unwrap(),
        );
        assert_eq!(parcel.parcel_type, ParcelType::Small);
        assert_eq!(parcel.cost(), 300);
    }

    #[test]
    fn a_medium_parcel_costs_8_dollars() {
        let parcel = Parcel::new(
            Dimensions::new(4.0, 10.0, 3.0).unwrap(),
            Weight::new(0.5).unwrap(),
        );
        assert_eq!(parcel.parcel_type, ParcelType::Medium);
        assert_eq!(parcel.cost(), 800);

        let parcel = Parcel::new(
            Dimensions::new(4.0, 3.0, 26.0).unwrap(),
            Weight::new(0.5).unwrap(),
        );
        assert_eq!(parcel.parcel_type, ParcelType::Medium);
        assert_eq!(parcel.cost(), 800);
    }

    #[test]
    fn a_large_parcel_costs_15_dollars() {
        let parcel = Parcel::new(
            Dimensions::new(4.0, 10.0, 50.0).unwrap(),
            Weight::new(0.5).unwrap(),
        );
        assert_eq!(parcel.parcel_type, ParcelType::Large);
        assert_eq!(parcel.cost(), 1500);

        let parcel = Parcel::new(
            Dimensions::new(4.0, 3.0, 99.9).unwrap(),
            Weight::new(0.5).unwrap(),
        );
        assert_eq!(parcel.parcel_type, ParcelType::Large);
        assert_eq!(parcel.cost(), 1500);
    }

    #[test]
    fn an_xl_parcel_costs_25_dollars() {
        let parcel = Parcel::new(
            Dimensions::new(4.0, 10.0, 100.0).unwrap(),
            Weight::new(0.5).unwrap(),
        );
        assert_eq!(parcel.parcel_type, ParcelType::XL);
        assert_eq!(parcel.cost(), 2500);

        let parcel = Parcel::new(
            Dimensions::new(4.0, 314.15, 100.0).unwrap(),
            Weight::new(0.5).unwrap(),
        );
        assert_eq!(parcel.parcel_type, ParcelType::XL);
        assert_eq!(parcel.cost(), 2500);
    }

    #[test]
    fn an_over_weight_limit_parcel_costs_2_dollars_more_per_kilogram() {
        let parcel = Parcel::new(
            Dimensions::new(1.0, 2.0, 3.0).unwrap(),
            Weight::new(4.23).unwrap(),
        );
        // 300+200*floor(4.23-1) = 900
        assert_eq!(parcel.cost(), 900);

        let parcel = Parcel::new(
            Dimensions::new(4.0, 10.0, 50.0).unwrap(),
            Weight::new(8.0).unwrap(),
        );
        // 1500+200*floor(8-6) = 1900
        assert_eq!(parcel.cost(), 1900);

        let parcel = Parcel::new(
            Dimensions::new(7.0, 2.0, 124.0).unwrap(),
            Weight::new(15.52).unwrap(),
        );
        // 2500+200*floor(15.52-10) = 3500
        assert_eq!(parcel.cost(), 3500);
    }

    #[test]
    fn a_heavy_parcel_costs_50_dollars() {
        let parcel = Parcel::new(
            Dimensions::new(4.0, 10.0, 50.0).unwrap(),
            Weight::new(50.0).unwrap(),
        );
        assert_eq!(parcel.cost(), 5000);
    }

    #[test]
    fn an_over_weight_limit_heavy_parcel_costs_1_dollars_more_per_kilogram() {
        let parcel = Parcel::new(
            Dimensions::new(4.0, 10.0, 50.0).unwrap(),
            Weight::new(51.0).unwrap(),
        );
        assert_eq!(parcel.cost(), 5100);
        let parcel = Parcel::new(
            Dimensions::new(4.0, 10.0, 50.0).unwrap(),
            Weight::new(63.14).unwrap(),
        );
        assert_eq!(parcel.cost(), 6300);
    }
}
