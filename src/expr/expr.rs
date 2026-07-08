#![warn(clippy::arithmetic_side_effects)]

use std::{
    iter,
    ops::{Add, Sub},
};

use crate::{
    error::{Error, Result},
    expr::{binop::BinOp, die::Die, inner::Inner, modifier::Modifier, scalar::Scalar},
    stats::{Distribution, Stats},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Expr {
    pub(super) inner: Inner,
    pub(super) mods: Modifier,
}

impl<E: Into<Inner>> From<E> for Expr {
    fn from(expr: E) -> Self {
        Self {
            inner: expr.into(),
            mods: Modifier::repeat(1),
        }
    }
}

impl Add for Expr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        BinOp::add(self.into(), rhs.into()).into()
    }
}

impl Sub for Expr {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        BinOp::sub(self.into(), rhs.into()).into()
    }
}

impl From<i32> for Expr {
    fn from(v: i32) -> Self {
        Scalar::new(v).into()
    }
}

impl Expr {
    pub fn die(sides: u32) -> Self {
        Die::new(sides).into()
    }

    pub fn scalar(v: i32) -> Self {
        Scalar::new(v).into()
    }

    pub fn repeat(self, n: u32) -> Result<Self> {
        Ok(Self {
            mods: self.mods.merge(Modifier::repeat(n))?,
            ..self
        })
    }

    pub const fn as_binop(&self) -> Option<&BinOp> {
        self.inner.as_binop()
    }

    pub fn into_binop(self) -> std::result::Result<BinOp, Self> {
        self.inner
            .into_binop()
            .map_err(|inner| Self { inner, ..self })
    }

    pub const fn as_die(&self) -> Option<&Die> {
        self.inner.as_die()
    }

    pub fn into_die(self) -> std::result::Result<Die, Self> {
        self.inner
            .into_die()
            .map_err(|inner| Self { inner, ..self })
    }

    pub const fn as_scalar(&self) -> Option<&Scalar> {
        self.inner.as_scalar()
    }

    pub fn into_scalar(self) -> std::result::Result<Scalar, Self> {
        self.inner
            .into_scalar()
            .map_err(|inner| Self { inner, ..self })
    }

    pub fn roll(&self) -> Result<i128> {
        let Self {
            inner,
            mods: Modifier { repeat },
        } = self;
        iter::repeat_with(|| inner.roll())
            .take(usize::try_from(*repeat)?)
            .sum()
    }
}

impl Stats for Expr {
    #[expect(clippy::arithmetic_side_effects)]
    fn distribution(&self) -> Result<Distribution> {
        let Self {
            inner,
            mods: Modifier { repeat },
        } = self;
        let mut distr = inner.distribution()?;
        for _ in 1..*repeat {
            distr = (distr.clone() + distr)?;
        }
        Ok(distr)
    }

    fn min(&self) -> Result<i128> {
        let Self {
            inner,
            mods: Modifier { repeat },
        } = self;
        inner
            .min()?
            .checked_mul(i128::from(*repeat))
            .ok_or(Error::Overflow)
    }

    fn max(&self) -> Result<i128> {
        let Self {
            inner,
            mods: Modifier { repeat },
        } = self;
        inner
            .max()?
            .checked_mul(i128::from(*repeat))
            .ok_or(Error::Overflow)
    }

    fn expected_value(&self) -> Result<f64> {
        let Self {
            inner,
            mods: Modifier { repeat },
        } = self;
        // EV(Σ_1_n X) = n * EV(X)
        Ok(inner.expected_value()? * f64::from(*repeat))
    }

    fn variance(&self) -> Result<f64> {
        let Self {
            inner,
            mods: Modifier { repeat },
        } = self;
        // Var(Σ_1_n X) = n * Var(X)
        Ok(inner.variance()? * f64::from(*repeat))
    }

    fn std_deviation(&self) -> Result<f64> {
        let Self {
            inner,
            mods: Modifier { repeat },
        } = self;
        // SD(Σ_1_n X) = sqrt(Var(Σ_1_n X)) = sqrt(n * Var(X)) = sqrt(n) * SD(X)
        Ok(inner.std_deviation()? * f64::from(*repeat).sqrt())
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use itertools::Itertools;
    use proptest::prelude::*;

    use super::*;
    use crate::expr::arbitrary::arb_expr;

    proptest! {
        #[test]
        fn roll_in_range(expr in arb_expr()) {
            let v = expr.roll().unwrap();
            let min = expr.min().unwrap();
            let max = expr.max().unwrap();
            assert!(min <= v, "{min} <= {v}");
            assert!(v <= max, "{v} <= {max}");
        }

        #[test]
        #[expect(clippy::cast_precision_loss)]
        fn ev_in_range(expr in arb_expr()) {
            let ev = expr.expected_value().unwrap();
            let min = expr.min().unwrap() as f64;
            let max = expr.max().unwrap() as f64;
            assert!(min <= ev, "{min} <= {ev}");
            assert!(ev <= max, "{ev} <= {max}");
        }


        #[test]
        fn var_std_dev(expr in arb_expr()) {
            let var = expr.variance().unwrap();
            let std_dev = expr.std_deviation().unwrap();
            assert_relative_eq!(var.sqrt(), std_dev, epsilon = 1e-6);
        }
    }

    #[test]
    fn two_d6() {
        let d = Expr::die(6).repeat(2).unwrap();
        assert_eq!(d.min().unwrap(), 2);
        assert_eq!(d.max().unwrap(), 12);
        assert_relative_eq!(d.expected_value().unwrap(), 7.0);
        assert_relative_eq!(d.variance().unwrap(), 70.0f64 / 12.0);
        assert_relative_eq!(d.std_deviation().unwrap(), (70.0f64 / 12.0).sqrt());
        assert_eq!(
            d.distribution()
                .unwrap()
                .iter()
                .sorted()
                .collect::<Vec<_>>(),
            vec![
                (2, 1),
                (3, 2),
                (4, 3),
                (5, 4),
                (6, 5),
                (7, 6),
                (8, 5),
                (9, 4),
                (10, 3),
                (11, 2),
                (12, 1)
            ]
        );
    }
}
