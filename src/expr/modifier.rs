use crate::expr::error::{Error, Result};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Modifier {
    pub(super) repeat: u32,
}

impl Modifier {
    pub const fn repeat(n: u32) -> Self {
        Self { repeat: n }
    }

    pub fn merge(self, other: Self) -> Result<Self> {
        Ok(Self {
            repeat: other
                .repeat
                .checked_mul(self.repeat)
                .ok_or(Error::Overflow)?,
        })
    }
}
