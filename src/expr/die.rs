#![warn(clippy::arithmetic_side_effects)]

use derive_more::Display;

use crate::{
    error::Result,
    stats::{Distribution, Stats},
};

#[derive(Copy, Clone, Debug, Display, Eq, PartialEq)]
#[display("d{sides}")]
pub struct Die {
    sides: u32,
}

impl Die {
    pub const fn new(sides: u32) -> Self {
        Self { sides }
    }

    pub const fn sides(self) -> u32 {
        self.sides
    }

    pub fn roll(self) -> u32 {
        if self.sides == 0 {
            0
        } else {
            rand::random_range(1..=self.sides)
        }
    }
}

impl Stats for Die {
    fn distribution(&self) -> Result<Distribution> {
        Ok((self.min()?..=self.max()?).collect())
    }

    #[expect(clippy::bool_to_int_with_if)]
    fn min(&self) -> Result<i128> {
        Ok(if self.sides == 0 { 0 } else { 1 })
    }

    fn max(&self) -> Result<i128> {
        Ok(if self.sides == 0 {
            0
        } else {
            self.sides.into()
        })
    }

    fn expected_value(&self) -> Result<f64> {
        Ok(if self.sides == 0 {
            0.0
        } else {
            // EV(dn)
            // = sum(x, 1, n) / n
            // = (n + 1) * n / 2n
            // = (n + 1) / 2
            f64::from(self.sides).midpoint(1.0)
        })
    }

    fn variance(&self) -> Result<f64> {
        Ok(if self.sides == 0 {
            0.0
        } else {
            // Var(dn)
            // = sum((x - EV)^2, 1, n) / n
            // = (n^2 - 1) / 12
            f64::from(self.sides).mul_add(f64::from(self.sides), -1.0) / 12.0
        })
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use itertools::Itertools;

    use super::*;

    #[test]
    fn zero() {
        let d = Die::new(0);
        assert_eq!(d.min().unwrap(), 0);
        assert_eq!(d.max().unwrap(), 0);
        assert_relative_eq!(d.expected_value().unwrap(), 0.0);
        assert_relative_eq!(d.variance().unwrap(), 0.0);
        assert_relative_eq!(d.std_deviation().unwrap(), 0.0);
        assert_eq!(
            d.distribution()
                .unwrap()
                .iter()
                .sorted()
                .collect::<Vec<_>>(),
            vec![(0, 1)]
        );
    }

    #[test]
    fn d6() {
        let d = Die::new(6);
        assert_eq!(d.min().unwrap(), 1);
        assert_eq!(d.max().unwrap(), 6);
        assert_relative_eq!(d.expected_value().unwrap(), 3.5);
        assert_relative_eq!(d.variance().unwrap(), 35.0f64 / 12.0);
        assert_relative_eq!(d.std_deviation().unwrap(), (35.0f64 / 12.0).sqrt());
        assert_eq!(
            d.distribution()
                .unwrap()
                .iter()
                .sorted()
                .collect::<Vec<_>>(),
            vec![(1, 1), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1)]
        );
    }
}
