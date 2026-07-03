use std::iter;

use derive_more::Display;

use crate::expr::{die::Die, inner::Inner};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct Modifier {
    #[cfg_attr(test, proptest(strategy = "0usize..256"))]
    repeat: usize,
}

#[derive(Clone, Debug, Display, Eq, PartialEq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[display("{}{inner}", mods.repeat)]
pub struct Expr {
    inner: Inner,
    mods: Modifier,
}

impl<E: Into<Inner>> From<E> for Expr {
    fn from(expr: E) -> Self {
        Self {
            inner: expr.into(),
            mods: Modifier { repeat: 1 },
        }
    }
}

impl Expr {
    pub fn die(sides: usize) -> Self {
        Die::new(sides).into()
    }

    pub const fn repeat(self, n: usize) -> Self {
        Self {
            mods: Modifier { repeat: n },
            ..self
        }
    }

    pub fn roll(&self) -> usize {
        let Self {
            inner,
            mods: Modifier { repeat },
        } = self;
        iter::repeat_with(|| inner.roll()).take(*repeat).sum()
    }

    pub fn min(&self) -> usize {
        let Self {
            inner,
            mods: Modifier { repeat },
        } = self;
        inner.min() * repeat
    }

    pub fn max(&self) -> usize {
        let Self {
            inner,
            mods: Modifier { repeat },
        } = self;
        inner.max() * repeat
    }

    pub fn expected_value(&self) -> f64 {
        let Self {
            inner,
            mods: Modifier { repeat },
        } = self;
        inner.expected_value() * *repeat as f64
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    fn arb_expr() -> impl Strategy<Value = Expr> {
        const MAX_DEPTH: u32 = 4;
        const DESIRED_SIZE: u32 = 64;
        const EXPECTED_BRANCH_SIZE: u32 = 8;

        let leaf = prop_oneof![(0usize..=256).prop_map(Expr::die)];
        leaf.prop_recursive(MAX_DEPTH, DESIRED_SIZE, EXPECTED_BRANCH_SIZE, |inner| {
            prop_oneof![inner.prop_flat_map(|expr| {
                (0usize..=256).prop_map(move |n| Expr::repeat(expr.clone(), n))
            })]
        })
    }

    proptest! {
        #[test]
        fn roll_in_range(expr in arb_expr()) {
            let v = expr.roll();
            let min = expr.min();
            let max = expr.max();
            assert!(min <= v, "{min} <= {v}");
            assert!(v <= max, "{v} <= {max}");
        }

        #[test]
        fn ev_in_range(expr in arb_expr()) {
            let ev = expr.expected_value();
            let min = expr.min() as f64;
            let max = expr.max() as f64;
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
}
