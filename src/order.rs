use crate::parcel::{Parcel, ParcelType};
use crate::shipping_speed::ShippingSpeed;

/// This is the entry point for making new orders and printing receipts.
#[derive(Debug)]
pub struct Order {
    pub parcels: Vec<Parcel>,
    pub shipping_speed: ShippingSpeed,
}

impl Order {
    pub fn new(parcels: Vec<Parcel>, shipping_speed: ShippingSpeed) -> Self {
        Self {
            parcels,
            shipping_speed,
        }
    }

    /// Total cost of an order after every modification is applied
    pub fn total_cost(&self) -> i32 {
        self.partial_total_cost() * self.shipping_speed.cost_multiplier()
    }

    /// Sum of all of the total costs of each parcel in the order
    pub fn partial_total_cost(&self) -> i32 {
        self.parcels.iter().map(|parcel| parcel.cost()).sum()
    }

    pub fn discount(&self) -> i32 {
        self.discount_for_type(ParcelType::Small, 4) + self.discount_for_type(ParcelType::Medium, 3)
    }

    fn discount_for_type(&self, parcel_type: ParcelType, nth_parcel_free: usize) -> i32 {
        let mut sorted_parcels: Vec<_> = self
            .parcels
            .iter()
            .filter(|parcel| parcel.parcel_type == parcel_type)
            .collect();
        sorted_parcels.sort_by_key(|parcel| parcel.cost());
        // reversed so highest value parcels are discounted first.
        sorted_parcels.reverse();

        let mut discount = 0;

        let mut number_discounted = sorted_parcels.len() / nth_parcel_free;
        for parcel in sorted_parcels {
            if number_discounted == 0 {
                break;
            }
            number_discounted -= 1;
            discount -= parcel.cost();
        }

        discount
    }
}

#[cfg(test)]
mod tests {
    use crate::dimensions::Dimensions;
    use crate::parcel::Parcel;
    use crate::shipping_speed::ShippingSpeed;

    use super::Order;
    use crate::weight::Weight;

    #[test]
    fn a_parcel_costs_double_with_speedy_shipping() {
        let parcels = vec![
            Parcel::new(
                Dimensions::new(4.0, 3.0, 26.0).unwrap(),
                Weight::new(0.5).unwrap(),
            ),
            Parcel::new(
                Dimensions::new(4.0, 3.0, 8.0).unwrap(),
                Weight::new(0.5).unwrap(),
            ),
        ];
        let order = Order::new(parcels, ShippingSpeed::Speedy);
        assert_eq!(order.partial_total_cost(), 1100);
        assert_eq!(order.total_cost(), 2200);
    }

    #[test]
    fn a_parcel_costs_the_the_same_with_normal_shipping() {
        let parcels = vec![
            Parcel::new(
                Dimensions::new(4.0, 3.0, 26.0).unwrap(),
                Weight::new(0.5).unwrap(),
            ),
            Parcel::new(
                Dimensions::new(4.0, 3.0, 8.0).unwrap(),
                Weight::new(0.5).unwrap(),
            ),
        ];
        let order = Order::new(parcels, ShippingSpeed::Normal);
        assert_eq!(order.partial_total_cost(), 1100);
        assert_eq!(order.total_cost(), 1100);
    }

    #[test]
    fn many_medium_parcels_are_discounted_appropriately() {
        let parcels = vec![
            Parcel::new(
                Dimensions::new(4.0, 3.0, 26.0).unwrap(),
                Weight::new(3.0).unwrap(),
            ),
            Parcel::new(
                Dimensions::new(4.0, 3.0, 32.0).unwrap(),
                Weight::new(3.0).unwrap(),
            ),
            Parcel::new(
                Dimensions::new(4.0, 3.0, 32.0).unwrap(),
                Weight::new(3.0).unwrap(),
            ),
            Parcel::new(
                Dimensions::new(4.0, 3.0, 45.0).unwrap(),
                Weight::new(4.0).unwrap(),
            ),
            Parcel::new(
                Dimensions::new(4.0, 3.0, 27.0).unwrap(),
                Weight::new(4.0).unwrap(),
            ),
            Parcel::new(
                Dimensions::new(4.0, 19.0, 5.0).unwrap(),
                Weight::new(4.0).unwrap(),
            ),
        ];
        let order = Order::new(parcels, ShippingSpeed::Normal);
        assert_eq!(order.discount(), -1800);
    }
}
