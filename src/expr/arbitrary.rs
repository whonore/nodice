use proptest::prelude::*;

use crate::expr::expr::Expr;

pub fn arb_expr() -> impl Strategy<Value = Expr> {
    arb_leaf().prop_recursive(4, 8, 8, |expr| {
        let expr = prop_oneof![arb_binop(expr.clone(), expr)];
        (expr, 0u32..256).prop_filter_map("repeat", |(expr, repeat)| expr.repeat(repeat).ok())
    })
}

pub fn arb_leaf() -> impl Strategy<Value = Expr> {
    prop_oneof![arb_die().prop_map_into()]
}

pub fn arb_binop(
    lhs: impl Strategy<Value = Expr>,
    rhs: impl Strategy<Value = Expr>,
) -> impl Strategy<Value = Expr> {
    (lhs, rhs)
        .prop_flat_map(|(lhs, rhs)| prop_oneof![Just(lhs.clone() + rhs.clone()), Just(lhs - rhs)])
}

pub fn arb_die() -> impl Strategy<Value = Expr> {
    (0u32..256).prop_map(Expr::die)
}
