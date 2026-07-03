use std::iter;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr {
    Die { sides: usize },
    Repeat { expr: Box<Self>, n: usize },
}

impl Expr {
    pub const fn die(sides: usize) -> Self {
        Self::Die { sides }
    }

    pub fn repeat(self, n: usize) -> Self {
        Self::Repeat {
            expr: Box::new(self),
            n,
        }
    }

    pub fn roll(&self) -> usize {
        match self {
            Self::Die { sides: 0 } => 0,
            Self::Die { sides } => rand::random_range(1..=*sides),
            Self::Repeat { expr, n } => iter::repeat_with(|| expr.roll()).take(*n).sum(),
        }
    }

    pub fn min(&self) -> usize {
        match self {
            Self::Die { sides: 0 } => 0,
            Self::Die { .. } => 1,
            Self::Repeat { expr, n } => expr.min() * n,
        }
    }

    pub fn max(&self) -> usize {
        match self {
            Self::Die { sides } => *sides,
            Self::Repeat { expr, n } => expr.max() * n,
        }
    }

    pub fn expected_value(&self) -> f64 {
        match self {
            Self::Die { sides: 0 } => 0.0,
            // EV(dn)
            // = sum(1, n) / n
            // = (n + 1) * n / 2n
            // = (n + 1) / 2
            Self::Die { sides } => (sides + 1) as f64 / 2.0,
            // EV(nX)
            // = n * EV(X)
            Self::Repeat { expr, n } => *n as f64 * expr.expected_value(),
        }
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
    }
}
