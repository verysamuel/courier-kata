#[derive(Debug, PartialOrd, PartialEq)]
#[non_exhaustive]
pub enum ShippingSpeed {
    Normal,
    Speedy,
}

impl ShippingSpeed {
    pub fn cost_multiplier(&self) -> i32 {
        match self {
            ShippingSpeed::Normal => 1,
            ShippingSpeed::Speedy => 2,
        }
    }
}
