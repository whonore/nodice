use derive_more::From;

use crate::expr::{binop::BinOp, die::Die, error::Result, scalar::Scalar};

#[derive(Clone, Debug, Eq, From, PartialEq)]
pub enum Inner {
    BinOp(BinOp),
    Die(Die),
    Scalar(Scalar),
}

impl Inner {
    pub const fn as_binop(&self) -> Option<&BinOp> {
        match self {
            Self::BinOp(binop) => Some(binop),
            _ => None,
        }
    }

    pub fn into_binop(self) -> std::result::Result<BinOp, Self> {
        match self {
            Self::BinOp(binop) => Ok(binop),
            _ => Err(self),
        }
    }

    pub const fn as_die(&self) -> Option<&Die> {
        match self {
            Self::Die(die) => Some(die),
            _ => None,
        }
    }

    pub fn into_die(self) -> std::result::Result<Die, Self> {
        match self {
            Self::Die(die) => Ok(die),
            _ => Err(self),
        }
    }

    pub const fn as_scalar(&self) -> Option<&Scalar> {
        match self {
            Self::Scalar(scalar) => Some(scalar),
            _ => None,
        }
    }

    pub fn into_scalar(self) -> std::result::Result<Scalar, Self> {
        match self {
            Self::Scalar(scalar) => Ok(scalar),
            _ => Err(self),
        }
    }

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

    pub fn variance(&self) -> f64 {
        match self {
            Self::BinOp(binop) => binop.variance(),
            Self::Die(die) => die.variance(),
            Self::Scalar(scalar) => scalar.variance(),
        }
    }

    pub fn std_deviation(&self) -> f64 {
        match self {
            Self::BinOp(binop) => binop.std_deviation(),
            Self::Die(die) => die.std_deviation(),
            Self::Scalar(scalar) => scalar.std_deviation(),
        }
    }
}
