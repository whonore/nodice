use std::{
    fmt, iter,
    ops::{Add, Sub},
};

use crate::expr::{Error, binop::BinOp, die::Die, error::Result, inner::Inner, scalar::Scalar};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Modifier {
    repeat: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Expr {
    inner: Inner,
    mods: Modifier,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            inner,
            mods: Modifier { repeat },
        } = self;

        if *repeat != 1 {
            write!(f, "{repeat}")?;
        }

        let skip_parens = match inner {
            Inner::BinOp(_) => false,
            Inner::Die(_) => true,
            Inner::Scalar(_) => *repeat == 1,
        };
        if skip_parens {
            write!(f, "{inner}")
        } else {
            write!(f, "({inner})")
        }
    }
}

impl<E: Into<Inner>> From<E> for Expr {
    fn from(expr: E) -> Self {
        Self {
            inner: expr.into(),
            mods: Modifier { repeat: 1 },
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

impl From<u32> for Expr {
    fn from(v: u32) -> Self {
        Scalar::new(v).into()
    }
}

#[warn(clippy::arithmetic_side_effects)]
impl Expr {
    pub fn die(sides: u32) -> Self {
        Die::new(sides).into()
    }

    pub fn scalar(v: u32) -> Self {
        Scalar::new(v).into()
    }

    pub fn repeat(self, n: u32) -> Result<Self> {
        Ok(Self {
            mods: Modifier {
                repeat: n.checked_mul(self.mods.repeat).ok_or(Error::Overflow)?,
            },
            ..self
        })
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
            let ev = expr.expected_value();
            let min = expr.min().unwrap() as f64;
            let max = expr.max().unwrap() as f64;
            assert!(min <= ev, "{min} <= {ev}");
            assert!(ev <= max, "{ev} <= {max}");
        }

        #[test]
        fn display_roundtrip(expr in arb_expr()) {
            let s = expr.to_string();
            match s.parse::<Expr>() {
                Ok(got) => assert_eq!(got, expr, "{expr:?} -> {s} -> {got:?}"),
                Err(err) => panic!("{expr:?} -> {s} -> {err}"),
            }
        }
    }

    macro_rules! check_display {
        ($e:expr, $expect:expr) => {
            let expr = $e.parse::<Expr>().unwrap();
            let got = expr.to_string();
            assert_eq!(got, $expect, "{} -> {got}", $e);
        };
    }

    #[test]
    fn display_scalar() {
        check_display!("6", "6");
        check_display!("(6)", "6");
        check_display!("1(6)", "6");
    }

    #[test]
    fn display_die() {
        check_display!("d6", "d6");
        check_display!("1d6", "d6");
        check_display!("(1d6)", "d6");
        check_display!("1(d6)", "d6");
    }

    #[test]
    fn display_binop() {
        check_display!("d6 + d4", "(d6 + d4)");
        check_display!("(d6 + d4)", "(d6 + d4)");
        check_display!("2(d6 + d4)", "2(d6 + d4)");
        check_display!("((d6 + d2) + d4)", "((d6 + d2) + d4)");
        check_display!("(d6 + (d2 + d4))", "(d6 + (d2 + d4))");
    }
}
