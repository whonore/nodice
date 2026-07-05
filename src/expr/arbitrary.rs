use proptest::prelude::*;

use crate::expr::expr::Expr;

pub fn arb_expr() -> impl Strategy<Value = Expr> {
    arb_leaf().prop_recursive(4, 8, 8, |expr| {
        let expr = arb_repeat(prop_oneof![arb_binop(expr.clone(), expr)]);
        (expr, 0u32..256).prop_filter_map("repeat", |(expr, repeat)| expr.repeat(repeat).ok())
    })
}

pub fn arb_binop(
    lhs: impl Strategy<Value = Expr>,
    rhs: impl Strategy<Value = Expr>,
) -> impl Strategy<Value = Expr> {
    (lhs, rhs)
        .prop_flat_map(|(lhs, rhs)| prop_oneof![Just(lhs.clone() + rhs.clone()), Just(lhs - rhs)])
}

pub fn arb_leaf() -> impl Strategy<Value = Expr> {
    arb_repeat(prop_oneof![arb_scalar(), arb_die()])
}

pub fn arb_scalar() -> impl Strategy<Value = Expr> {
    (-128..128).prop_map(Expr::scalar)
}

pub fn arb_die() -> impl Strategy<Value = Expr> {
    // It's sometimes important to choose dice with the same number of sides,
    // which small helps with.
    let small = 1u32..8;
    let large = 8u32..256;
    prop_oneof![1 => Just(0u32), 3 => small, 1 => large].prop_map(Expr::die)
}

pub fn arb_repeat(expr: impl Strategy<Value = Expr>) -> impl Strategy<Value = Expr> {
    (0u32..8, expr).prop_filter_map("repeat", |(repeat, expr)| expr.repeat(repeat).ok())
}
