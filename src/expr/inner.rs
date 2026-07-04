use derive_more::From;

use crate::expr::{binop::BinOp, die::Die, error::Result, scalar::Scalar};

#[derive(Clone, Debug, Eq, From, PartialEq)]
pub enum Inner {
    BinOp(BinOp),
    Die(Die),
    Scalar(Scalar),
}

impl Inner {
    pub fn roll(&self) -> Result<i128> {
        match self {
            Self::BinOp(binop) => binop.roll(),
            Self::Die(die) => Ok(die.roll().into()),
            Self::Scalar(scalar) => Ok(scalar.roll().into()),
        }
    }

    pub fn min(&self) -> Result<i128> {
        match self {
            Self::BinOp(binop) => binop.min(),
            Self::Die(die) => Ok(die.min().into()),
            Self::Scalar(scalar) => Ok(scalar.min().into()),
        }
    }

    pub fn max(&self) -> Result<i128> {
        match self {
            Self::BinOp(binop) => binop.max(),
            Self::Die(die) => Ok(die.max().into()),
            Self::Scalar(scalar) => Ok(scalar.max().into()),
        }
    }

    pub fn expected_value(&self) -> f64 {
        match self {
            Self::BinOp(binop) => binop.expected_value(),
            Self::Die(die) => die.expected_value(),
            Self::Scalar(scalar) => scalar.expected_value(),
        }
    }
}
