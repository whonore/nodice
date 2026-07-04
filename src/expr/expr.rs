use std::{
    iter,
    ops::{Add, Sub},
};

use crate::expr::{
    Error, binop::BinOp, die::Die, error::Result, inner::Inner, modifier::Modifier, scalar::Scalar,
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

#[warn(clippy::arithmetic_side_effects)]
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

    pub fn min(&self) -> Result<i128> {
        let Self {
            inner,
            mods: Modifier { repeat },
        } = self;
        inner
            .min()?
            .checked_mul(i128::from(*repeat))
            .ok_or(Error::Overflow)
    }

    pub fn max(&self) -> Result<i128> {
        let Self {
            inner,
            mods: Modifier { repeat },
        } = self;
        inner
            .max()?
            .checked_mul(i128::from(*repeat))
            .ok_or(Error::Overflow)
    }

    pub fn expected_value(&self) -> f64 {
        let Self {
            inner,
            mods: Modifier { repeat },
        } = self;
        inner.expected_value() * f64::from(*repeat)
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

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
            let ev = expr.expected_value();
            let min = expr.min().unwrap() as f64;
            let max = expr.max().unwrap() as f64;
            assert!(min <= ev, "{min} <= {ev}");
            assert!(ev <= max, "{ev} <= {max}");
        }
    }
}
