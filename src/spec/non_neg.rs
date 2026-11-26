use serde::Deserialize;

//Derive an implementation that requires the
// null ration provided to not be a negative number
// Because we are validating on load there is no need to
// check in the application at multiple stages
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Deserialize)]
pub struct NonNegative(f64);

impl NonNegative {
    pub fn new(value: f64) -> Option<Self> {
        // `>= 0.0` is false for NaN, so this also rejects NaN.
        // Note: -0.0 >= 0.0 is true, so negative zero is allowed.
        if value > 0.0 && value.is_finite() {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn get(self) -> f64 {
        self.0
    }
}
