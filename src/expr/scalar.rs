#![warn(clippy::arithmetic_side_effects)]

use std::iter;

use derive_more::{Display, From};

use crate::{
    error::Result,
    stats::{Distribution, Stats},
};

#[derive(Copy, Clone, Debug, Display, Eq, From, PartialEq)]
#[display("{_0}")]
pub struct Scalar(i32);

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
}

impl Stats for Scalar {
    fn distribution(&self) -> Result<Distribution> {
        Ok(iter::once(self.value()).collect())
    }

    fn min(&self) -> Result<i128> {
        Ok(self.value().into())
    }

    fn max(&self) -> Result<i128> {
        Ok(self.value().into())
    }

    fn expected_value(&self) -> Result<f64> {
        Ok(self.value().into())
    }

    fn variance(&self) -> Result<f64> {
        Ok(0.0)
    }

    fn std_deviation(&self) -> Result<f64> {
        Ok(0.0)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use itertools::Itertools;

    use super::*;

    #[test]
    fn two() {
        let v = Scalar::new(2);
        assert_eq!(v.min().unwrap(), 2);
        assert_eq!(v.max().unwrap(), 2);
        assert_relative_eq!(v.expected_value().unwrap(), 2.0);
        assert_relative_eq!(v.variance().unwrap(), 0.0);
        assert_relative_eq!(v.std_deviation().unwrap(), 0.0);
        assert_eq!(
            v.distribution()
                .unwrap()
                .into_iter()
                .sorted()
                .collect::<Vec<_>>(),
            vec![2]
        );
    }
}
