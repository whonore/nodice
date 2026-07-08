#![warn(clippy::arithmetic_side_effects)]

use derive_more::Display;

use crate::{
    error::{Error, Result},
    expr::Expr,
    stats::{Distribution, Stats},
};

#[derive(Copy, Clone, Debug, Display, Eq, PartialEq)]
pub enum Op {
    #[display("+")]
    Add,
    #[display("-")]
    Sub,
}

impl Op {
    #[expect(clippy::arithmetic_side_effects)]
    pub fn apply(self, lhs: Expr, rhs: Expr) -> Expr {
        match self {
            Self::Add => lhs + rhs,
            Self::Sub => lhs - rhs,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BinOp {
    pub(super) lhs: Box<Expr>,
    pub(super) rhs: Box<Expr>,
    pub(super) op: Op,
}

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
}

impl Stats for BinOp {
    #[expect(clippy::arithmetic_side_effects)]
    fn distribution(&self) -> Result<Distribution> {
        let Self { lhs, rhs, op } = self;
        match op {
            Op::Add => lhs.distribution()? + rhs.distribution()?,
            Op::Sub => lhs.distribution()? - rhs.distribution()?,
        }
    }

    fn min(&self) -> Result<i128> {
        let Self { lhs, rhs, op } = self;
        match op {
            Op::Add => lhs.min()?.checked_add(rhs.min()?),
            Op::Sub => lhs.min()?.checked_sub(rhs.max()?),
        }
        .ok_or(Error::Overflow)
    }

    fn max(&self) -> Result<i128> {
        let Self { lhs, rhs, op } = self;
        match op {
            Op::Add => lhs.max()?.checked_add(rhs.max()?),
            Op::Sub => lhs.max()?.checked_sub(rhs.min()?),
        }
        .ok_or(Error::Overflow)
    }

    fn expected_value(&self) -> Result<f64> {
        let Self { lhs, rhs, op } = self;
        match op {
            // EV(X + Y) = EV(X) + EV(Y)
            Op::Add => Ok(lhs.expected_value()? + rhs.expected_value()?),
            // EV(X - Y) = EV(X) - EV(Y)
            Op::Sub => Ok(lhs.expected_value()? - rhs.expected_value()?),
        }
    }

    fn variance(&self) -> Result<f64> {
        let Self { lhs, rhs, op } = self;
        match op {
            // Var(X + Y) = Var(X) + Var(Y) + 2Cov(X, Y) = Var(X) + Var(Y)
            // Var(X - Y) = Var(X) + Var(Y) - 2Cov(X, Y) = Var(X) - Var(Y)
            Op::Add | Op::Sub => Ok(lhs.variance()? + rhs.variance()?),
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use itertools::Itertools;

    use crate::expr::die::Die;

    use super::*;

    #[test]
    fn d6_add_d4() {
        let d1 = Die::new(6);
        let d2 = Die::new(4);
        let e = BinOp::add(Box::new(d1.into()), Box::new(d2.into()));
        assert_eq!(e.min().unwrap(), 2);
        assert_eq!(e.max().unwrap(), 10);
        assert_relative_eq!(e.expected_value().unwrap(), 6.0);
        assert_relative_eq!(e.variance().unwrap(), (50.0f64 / 12.0));
        assert_relative_eq!(e.std_deviation().unwrap(), (50.0f64 / 12.0).sqrt());
        assert_eq!(
            e.distribution()
                .unwrap()
                .into_iter()
                .sorted()
                .collect::<Vec<_>>(),
            vec![
                2, 3, 3, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 9, 9, 10
            ]
        );
    }

    #[test]
    fn d6_sub_d4() {
        let d1 = Die::new(6);
        let d2 = Die::new(4);
        let e = BinOp::sub(Box::new(d1.into()), Box::new(d2.into()));
        assert_eq!(e.min().unwrap(), -3);
        assert_eq!(e.max().unwrap(), 5);
        assert_relative_eq!(e.expected_value().unwrap(), 1.0);
        assert_relative_eq!(e.variance().unwrap(), (50.0f64 / 12.0));
        assert_relative_eq!(e.std_deviation().unwrap(), (50.0f64 / 12.0).sqrt());
        assert_eq!(
            e.distribution()
                .unwrap()
                .into_iter()
                .sorted()
                .collect::<Vec<_>>(),
            vec![
                -3, -2, -2, -1, -1, -1, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 5
            ]
        );
    }
}
