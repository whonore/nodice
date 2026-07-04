use derive_more::{Display, From};

#[derive(Copy, Clone, Debug, Display, Eq, From, PartialEq)]
#[display("{_0}")]
pub struct Scalar(i32);

#[warn(clippy::arithmetic_side_effects)]
impl Scalar {
    pub const fn new(v: i32) -> Self {
        Self(v)
    }

    pub const fn value(self) -> i32 {
        self.0
    }

    pub const fn roll(self) -> i32 {
        self.0
    }

    pub const fn min(self) -> i32 {
        self.0
    }

    pub const fn max(self) -> i32 {
        self.0
    }

    pub fn expected_value(self) -> f64 {
        self.0.into()
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn two() {
        let v = Scalar::new(2);
        assert_eq!(v.min(), 2);
        assert_eq!(v.max(), 2);
        assert_relative_eq!(v.expected_value(), 2.0);
    }
}
