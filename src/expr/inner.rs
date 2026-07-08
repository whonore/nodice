#![warn(clippy::arithmetic_side_effects)]

use derive_more::From;

use crate::{
    error::Result,
    expr::{binop::BinOp, die::Die, scalar::Scalar},
    stats::{Distribution, Stats},
};

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
}

impl Stats for Inner {
    fn distribution(&self) -> Result<Distribution> {
        match self {
            Self::BinOp(binop) => binop.distribution(),
            Self::Die(die) => die.distribution(),
            Self::Scalar(scalar) => scalar.distribution(),
        }
    }

    fn min(&self) -> Result<i128> {
        match self {
            Self::BinOp(binop) => binop.min(),
            Self::Die(die) => die.min(),
            Self::Scalar(scalar) => scalar.min(),
        }
    }

    fn max(&self) -> Result<i128> {
        match self {
            Self::BinOp(binop) => binop.max(),
            Self::Die(die) => die.max(),
            Self::Scalar(scalar) => scalar.max(),
        }
    }

    fn expected_value(&self) -> Result<f64> {
        match self {
            Self::BinOp(binop) => binop.expected_value(),
            Self::Die(die) => die.expected_value(),
            Self::Scalar(scalar) => scalar.expected_value(),
        }
    }

    fn variance(&self) -> Result<f64> {
        match self {
            Self::BinOp(binop) => binop.variance(),
            Self::Die(die) => die.variance(),
            Self::Scalar(scalar) => scalar.variance(),
        }
    }

    fn std_deviation(&self) -> Result<f64> {
        match self {
            Self::BinOp(binop) => binop.std_deviation(),
            Self::Die(die) => die.std_deviation(),
            Self::Scalar(scalar) => scalar.std_deviation(),
        }
    }
}
