use crate::expr::{
    Expr,
    binop::{BinOp, Op},
    error::{Error, Result},
    scalar::Scalar,
};

#[warn(clippy::arithmetic_side_effects)]
impl Expr {
    pub fn simplify(self) -> Result<Self> {
        self.fold_constants()
    }

    fn fold_constants(self) -> Result<Self> {
        match self.inner.into_binop() {
            Ok(BinOp { lhs, rhs, op }) => {
                let lhs = lhs.fold_constants()?;
                let rhs = rhs.fold_constants()?;
                if let Some(lhs_scalar) = lhs.as_scalar()
                    && let Some(rhs_scalar) = rhs.as_scalar()
                    && lhs.mods == rhs.mods
                {
                    let mods = lhs.mods.merge(self.mods)?;
                    let lhs = lhs_scalar.value();
                    let rhs = rhs_scalar.value();
                    let v = match op {
                        Op::Add => lhs.checked_add(rhs),
                        Op::Sub => lhs.checked_sub(rhs),
                    }
                    .ok_or(Error::Overflow)?;
                    Ok(Self {
                        inner: Scalar::new(v).into(),
                        mods,
                    })
                } else {
                    Ok(Self {
                        inner: BinOp {
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                            op,
                        }
                        .into(),
                        ..self
                    })
                }
            }
            Err(inner) => Ok(Self { inner, ..self }),
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use proptest::prelude::*;

    use super::*;
    use crate::expr::arbitrary::arb_expr;

    macro_rules! assert_expr_equiv {
        ($e1:expr, $e2:expr) => {
            assert_eq!($e1.min(), $e2.min(), "{} != {}", $e1, $e2);
            assert_eq!($e1.max(), $e2.max(), "{} != {}", $e1, $e2);
            assert_relative_eq!($e1.expected_value(), $e2.expected_value());
        };
    }

    proptest! {
        #[test]
        fn simplify_equiv(expr in arb_expr()) {
            let simpl = expr.clone().simplify().unwrap();
            assert_expr_equiv!(simpl, expr);
        }

        #[test]
        fn simplify_fixpoint(expr in arb_expr()) {
            let simpl1 = expr.simplify().unwrap();
            let simpl2 = simpl1.clone().simplify().unwrap();
            assert_eq!(simpl1, simpl2);
        }
    }

    macro_rules! check_simplify {
        ($e:expr, $expect:expr) => {
            let expr = $e.parse::<Expr>().unwrap();
            let got = expr.simplify().unwrap().to_string();
            assert_eq!(got, $expect);
        };
    }

    #[test]
    fn simplify_constants() {
        check_simplify!("6", "6");
        check_simplify!("6 + 1", "7");
        check_simplify!("6 - 2 + 1", "5");
        check_simplify!("6 - (2 + 1)", "3");
        check_simplify!("6 - (2 + 1) + d6", "3 + d6");
    }
}
