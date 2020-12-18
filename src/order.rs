use crate::parcel::Parcel;
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
}

#[cfg(test)]
mod tests {
    use crate::dimensions::Dimensions;
    use crate::parcel::Parcel;
    use crate::shipping_speed::ShippingSpeed;

    use super::Order;

    #[test]
    fn a_parcel_costs_double_with_speedy_shipping() {
        let parcels = vec![
            Parcel::new(Dimensions::new(4.0, 3.0, 26.0).unwrap()),
            Parcel::new(Dimensions::new(4.0, 3.0, 8.0).unwrap()),
        ];
        let order = Order::new(parcels, ShippingSpeed::Speedy);
        assert_eq!(order.partial_total_cost(), 1100);
        assert_eq!(order.total_cost(), 2200);
    }

    #[test]
    fn a_parcel_costs_the_the_same_with_normal_shipping() {
        let parcels = vec![
            Parcel::new(Dimensions::new(4.0, 3.0, 26.0).unwrap()),
            Parcel::new(Dimensions::new(4.0, 3.0, 8.0).unwrap()),
        ];
        let order = Order::new(parcels, ShippingSpeed::Normal);
        assert_eq!(order.partial_total_cost(), 1100);
        assert_eq!(order.total_cost(), 1100);
    }
}
