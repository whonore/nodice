use derive_more::Display;

use crate::expr::{Error, error::Result, expr::Expr};

#[derive(Copy, Clone, Debug, Display, Eq, PartialEq)]
pub enum Op {
    #[display("+")]
    Add,
    #[display("-")]
    Sub,
}

impl Op {
    pub fn apply(self, lhs: Expr, rhs: Expr) -> Expr {
        match self {
            Self::Add => lhs + rhs,
            Self::Sub => lhs - rhs,
        }
    }
}

#[derive(Clone, Debug, Display, Eq, PartialEq)]
#[display("{lhs} {op} {rhs}")]
pub struct BinOp {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
    op: Op,
}

#[warn(clippy::arithmetic_side_effects)]
impl BinOp {
    pub const fn add(lhs: Box<Expr>, rhs: Box<Expr>) -> Self {
        Self {
            lhs,
            rhs,
            op: Op::Add,
        }
    }

    pub const fn sub(lhs: Box<Expr>, rhs: Box<Expr>) -> Self {
        Self {
            lhs,
            rhs,
            op: Op::Sub,
        }
    }

    pub fn roll(&self) -> Result<i128> {
        let Self { lhs, rhs, op } = self;
        let lhs = lhs.roll()?;
        let rhs = rhs.roll()?;
        match op {
            Op::Add => lhs.checked_add(rhs),
            Op::Sub => lhs.checked_sub(rhs),
        }
        .ok_or(Error::Overflow)
    }

    pub fn min(&self) -> Result<i128> {
        let Self { lhs, rhs, op } = self;
        match op {
            Op::Add => lhs.min()?.checked_add(rhs.min()?),
            Op::Sub => lhs.min()?.checked_sub(rhs.max()?),
        }
        .ok_or(Error::Overflow)
    }

    pub fn max(&self) -> Result<i128> {
        let Self { lhs, rhs, op } = self;
        match op {
            Op::Add => lhs.max()?.checked_add(rhs.max()?),
            Op::Sub => lhs.max()?.checked_sub(rhs.min()?),
        }
        .ok_or(Error::Overflow)
    }

    pub fn expected_value(&self) -> f64 {
        let Self { lhs, rhs, op } = self;
        match op {
            // EV(X + Y) = EV(X) + EV(Y)
            Op::Add => lhs.expected_value() + rhs.expected_value(),
            // EV(X - Y) = EV(X) - EV(Y)
            Op::Sub => lhs.expected_value() - rhs.expected_value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::expr::die::Die;

    use super::*;

    #[test]
    fn d6_add_d4() {
        let d1 = Die::new(6);
        let d2 = Die::new(4);
        let e = BinOp::add(Box::new(d1.into()), Box::new(d2.into()));
        assert_eq!(e.min().unwrap(), 2);
        assert_eq!(e.max().unwrap(), 10);
        assert_relative_eq!(e.expected_value(), 6.0);
    }

    #[test]
    fn d6_sub_d4() {
        let d1 = Die::new(6);
        let d2 = Die::new(4);
        let e = BinOp::sub(Box::new(d1.into()), Box::new(d2.into()));
        assert_eq!(e.min().unwrap(), -3);
        assert_eq!(e.max().unwrap(), 5);
        assert_relative_eq!(e.expected_value(), 1.0);
    }
}
