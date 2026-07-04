use derive_more::Display;

#[derive(Copy, Clone, Debug, Display, Eq, PartialEq)]
#[display("d{sides}")]
pub struct Die {
    sides: u32,
}

#[warn(clippy::arithmetic_side_effects)]
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

    pub const fn min(self) -> u32 {
        if self.sides == 0 { 0 } else { 1 }
    }

    pub const fn max(self) -> u32 {
        if self.sides == 0 { 0 } else { self.sides }
    }

    pub fn expected_value(self) -> f64 {
        if self.sides == 0 {
            0.0
        } else {
            // EV(dn)
            // = sum(1, n) / n
            // = (n + 1) * n / 2n
            // = (n + 1) / 2
            f64::from(self.sides).midpoint(1.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn zero() {
        let d = Die::new(0);
        assert_eq!(d.min(), 0);
        assert_eq!(d.max(), 0);
        assert_relative_eq!(d.expected_value(), 0.0);
    }

    #[test]
    fn d6() {
        let d = Die::new(6);
        assert_eq!(d.min(), 1);
        assert_eq!(d.max(), 6);
        assert_relative_eq!(d.expected_value(), 3.5);
    }
}
