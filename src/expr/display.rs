use std::fmt;

use crate::expr::{Expr, binop::BinOp, inner::Inner, modifier::Modifier};

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, false)
    }
}

impl fmt::Display for Inner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, false, false)
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f)
    }
}

impl Expr {
    fn display(&self, f: &mut fmt::Formatter<'_>, in_rhs: bool) -> fmt::Result {
        let Self {
            inner,
            mods: Modifier { repeat },
        } = self;

        let has_repeat = *repeat != 1;
        if has_repeat {
            write!(f, "{repeat}")?;
        }

        inner.display(f, has_repeat, in_rhs)
    }
}

impl Inner {
    fn display(&self, f: &mut fmt::Formatter<'_>, has_repeat: bool, in_rhs: bool) -> fmt::Result {
        match self {
            Self::BinOp(binop) if has_repeat || in_rhs => {
                write!(f, "(")?;
                binop.display(f)?;
                write!(f, ")")
            }
            Self::BinOp(binop) => binop.display(f),
            Self::Die(die) => write!(f, "{die}"),
            Self::Scalar(scalar) if !has_repeat => write!(f, "{scalar}"),
            Self::Scalar(scalar) => write!(f, "({scalar})"),
        }
    }
}

impl BinOp {
    fn display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { lhs, rhs, op } = self;
        lhs.display(f, false)?;
        write!(f, " {op} ")?;
        rhs.display(f, true)
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::expr::arbitrary::arb_expr;

    proptest! {
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
            assert_eq!(got, $expect, "{} -> {expr:?}", $e);
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
        check_display!("d6 + d4", "d6 + d4");
        check_display!("(d6 + d4)", "d6 + d4");
        check_display!("2(d6 + d4)", "2(d6 + d4)");
        check_display!("d6 + d2 + d4", "d6 + d2 + d4");
        check_display!("d6 + (d2 + d4)", "d6 + (d2 + d4)");
        check_display!("d6 + 2(d2 + d4)", "d6 + 2(d2 + d4)");
        check_display!("(d6 + d2) + d4", "d6 + d2 + d4");
        check_display!("2(d6 + d2) + d4", "2(d6 + d2) + d4");
    }
}
