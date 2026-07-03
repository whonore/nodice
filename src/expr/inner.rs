use derive_more::{Display, From};

use crate::expr::die::Die;

#[derive(Clone, Debug, Display, Eq, From, PartialEq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum Inner {
    Die(Die),
}

impl Inner {
    pub fn roll(&self) -> usize {
        match self {
            Self::Die(die) => die.roll(),
        }
    }

    pub const fn min(&self) -> usize {
        match self {
            Self::Die(die) => die.min(),
        }
    }

    pub const fn max(&self) -> usize {
        match self {
            Self::Die(die) => die.max(),
        }
    }

    pub fn expected_value(&self) -> f64 {
        match self {
            Self::Die(die) => die.expected_value(),
        }
    }
}
