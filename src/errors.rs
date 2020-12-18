use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParcelError {
    #[error(
        "Dimensions must all be positive but `({0}, {1}, {2})` contains a non-positive dimension"
    )]
    NonPositiveDimensions(f32, f32, f32),
    #[error("Weight must be positive but `{0}` is a non-positive weight")]
    NonPositiveWeight(f32),
}
