use crate::expr::{
    Expr,
    binop::{BinOp, Op},
    error::{Error, Result},
    inner::Inner,
    modifier::Modifier,
    scalar::Scalar,
};

#[warn(clippy::arithmetic_side_effects)]
impl Expr {
    pub fn simplify(self) -> Result<Self> {
        self.distribute_repeats()?.fold_constants()
    }

    fn distribute_repeats(self) -> Result<Self> {
        if self.mods.repeat == 0 {
            return Ok(Self::scalar(0));
        }

        match self.inner {
            // R(X + Y) => R(X) + R(Y)
            Inner::BinOp(BinOp { lhs, rhs, op }) => {
                let lhs = lhs.repeat(self.mods.repeat)?.distribute_repeats()?;
                let rhs = rhs.repeat(self.mods.repeat)?.distribute_repeats()?;
                Ok(BinOp {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    op,
                }
                .into())
            }
            Inner::Die(..) => Ok(self),
            // R(N) => R * N
            Inner::Scalar(scalar) => Ok(Self::scalar(
                scalar
                    .value()
                    .checked_mul(i32::try_from(self.mods.repeat)?)
                    .ok_or(Error::Overflow)?,
            )),
        }
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
                } else if let Some(lhs_die) = lhs.as_die()
                    && let Some(rhs_die) = rhs.as_die()
                    && lhs_die.sides() == rhs_die.sides()
                {
                    let lhs_repeats = lhs.mods.repeat;
                    let rhs_repeats = rhs.mods.repeat;
                    let repeats = match op {
                        Op::Add => lhs_repeats.checked_add(rhs_repeats),
                        Op::Sub if lhs_repeats > rhs_repeats => {
                            lhs_repeats.checked_sub(rhs_repeats)
                        }
                        Op::Sub => {
                            return Ok(Self {
                                inner: BinOp {
                                    lhs: Box::new(lhs),
                                    rhs: Box::new(rhs),
                                    op,
                                }
                                .into(),
                                ..self
                            });
                        }
                    }
                    .ok_or(Error::Overflow)?;
                    let mods = Modifier::repeat(repeats).merge(self.mods)?;
                    Ok(Self { mods, ..lhs })
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

    fn only_leaves_repeat(expr: &Expr) -> bool {
        match &expr.inner {
            Inner::BinOp(BinOp { lhs, rhs, .. }) => {
                expr.mods.repeat == 1 && only_leaves_repeat(lhs) && only_leaves_repeat(rhs)
            }
            Inner::Die(..) | Inner::Scalar(..) => true,
        }
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

        #[test]
        fn simplify_only_leaves_repeat(expr in arb_expr()) {
            let simpl = expr.simplify().unwrap();
            assert!(only_leaves_repeat(&simpl));
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
    fn simplify_repeat() {
        check_simplify!("2(3)", "6");
        check_simplify!("2(3(4))", "24");
        check_simplify!("2(3d6)", "6d6");
        check_simplify!("2(d6 + 2d8)", "2d6 + 4d8");
        check_simplify!("2(1 - 3(d6 + 2d8))", "2 - (6d6 + 12d8)");
    }

    #[test]
    fn simplify_repeat_0() {
        check_simplify!("0d6", "0");
        check_simplify!("2(0d6)", "0");
        check_simplify!("0d6 + 1", "1");
        check_simplify!("0(1d6 + 1)", "0");
    }

    #[test]
    fn simplify_constants() {
        check_simplify!("6", "6");
        check_simplify!("6 + 1", "7");
        check_simplify!("6 - 2 + 1", "5");
        check_simplify!("6 - (2 + 1)", "3");
        check_simplify!("6 - (2 + 1) + d6", "3 + d6");
    }

    #[test]
    fn simplify_dice() {
        check_simplify!("6d6", "6d6");
        check_simplify!("6d6 + d6", "7d6");
        check_simplify!("6d6 - 2d6 + d6", "5d6");
        check_simplify!("6d6 - (2d6 + d6)", "3d6");
        check_simplify!("6d6 - (2d6 + d6) + 6", "3d6 + 6");
        check_simplify!("d6 + d4", "d6 + d4");
        check_simplify!("d6 - d6", "d6 - d6");
        check_simplify!("d6 - 2d6", "d6 - 2d6");
    }
}
