#[cfg(test)]
mod arbitrary;
mod binop;
mod die;
mod display;
mod error;
mod expr;
mod inner;
mod modifier;
mod parser;
mod scalar;
mod simplify;

pub use crate::expr::{
    error::{Error, Result},
    expr::Expr,
};
