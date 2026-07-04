#[cfg(test)]
mod arbitrary;
mod binop;
mod die;
mod error;
mod expr;
mod inner;
mod parser;

pub use crate::expr::{
    error::{Error, Result},
    expr::Expr,
};
