use derive_more::{Display, From};

use crate::expr::{binop::BinOp, die::Die, error::Result};

#[derive(Clone, Debug, Display, Eq, From, PartialEq)]
pub enum Inner {
    Die(Die),
    BinOp(BinOp),
}

impl Inner {
    pub fn roll(&self) -> Result<i128> {
        match self {
            Self::Die(die) => Ok(die.roll().into()),
            Self::BinOp(binop) => binop.roll(),
        }
    }

    pub fn min(&self) -> Result<i128> {
        match self {
            Self::Die(die) => Ok(die.min().into()),
            Self::BinOp(binop) => binop.min(),
        }
    }

    pub fn max(&self) -> Result<i128> {
        match self {
            Self::Die(die) => Ok(die.max().into()),
            Self::BinOp(binop) => binop.max(),
        }
    }

    pub fn expected_value(&self) -> f64 {
        match self {
            Self::Die(die) => die.expected_value(),
            Self::BinOp(binop) => binop.expected_value(),
        }
    }
}
