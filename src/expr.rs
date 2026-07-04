#[cfg(test)]
mod arbitrary;
mod binop;
mod die;
mod display;
mod error;
mod expr;
mod inner;
mod parser;
mod scalar;

pub use crate::expr::{
    error::{Error, Result},
    expr::Expr,
};
