#![warn(clippy::arithmetic_side_effects)]

use std::ops::{Add, Sub};

use ahash::AHashMap;
use itertools::Itertools;

use crate::error::{Error, Result};

#[derive(Clone, Debug)]
pub struct Distribution(AHashMap<i128, usize>);

impl<V: Into<i128>> FromIterator<V> for Distribution {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let counts = iter.into_iter().map(Into::into).counts();
        Self(counts.into_iter().collect())
    }
}

impl Add for Distribution {
    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = AHashMap::<i128, usize>::with_capacity(
            self.0
                .len()
                .checked_mul(rhs.0.len())
                .ok_or(Error::Overflow)?,
        );
        for (x, x_count) in &self.0 {
            for (y, y_count) in &rhs.0 {
                let z = x.checked_add(*y).ok_or(Error::Overflow)?;
                let z_count = x_count.checked_mul(*y_count).ok_or(Error::Overflow)?;
                let entry = res.entry(z).or_default();
                *entry = entry.checked_add(z_count).ok_or(Error::Overflow)?;
            }
        }
        Ok(Self(res))
    }
}

impl Sub for Distribution {
    type Output = Result<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = AHashMap::<i128, usize>::with_capacity(
            self.0
                .len()
                .checked_mul(rhs.0.len())
                .ok_or(Error::Overflow)?,
        );
        for (x, x_count) in &self.0 {
            for (y, y_count) in &rhs.0 {
                let z = x.checked_sub(*y).ok_or(Error::Overflow)?;
                let z_count = x_count.checked_mul(*y_count).ok_or(Error::Overflow)?;
                let entry = res.entry(z).or_default();
                *entry = entry.checked_add(z_count).ok_or(Error::Overflow)?;
            }
        }
        Ok(Self(res))
    }
}

impl Distribution {
    pub fn iter(&self) -> impl Iterator<Item = (i128, usize)> {
        self.0.iter().map(|(v, count)| (*v, *count))
    }

    pub fn size(&self) -> Result<f64> {
        Ok(f64::from(u32::try_from(
            self.iter().map(|(_, count)| count).sum::<usize>(),
        )?))
    }

    pub fn min(&self) -> Result<i128> {
        self.iter()
            .map(|(v, _)| v)
            .min()
            .ok_or(Error::EmptyDistribution)
    }

    pub fn max(&self) -> Result<i128> {
        self.iter()
            .map(|(v, _)| v)
            .max()
            .ok_or(Error::EmptyDistribution)
    }

    pub fn expected_value(&self) -> Result<f64> {
        let sum = self.iter().try_fold(0i128, |ev, (v, count)| {
            ev.checked_add(
                v.checked_mul(i128::try_from(count)?)
                    .ok_or(Error::Overflow)?,
            )
            .ok_or(Error::Overflow)
        })?;
        Ok(f64::from(i32::try_from(sum)?) / self.size()?)
    }

    pub fn variance(&self) -> Result<f64> {
        let ev = self.expected_value()?;
        let sum = self
            .iter()
            .try_fold(0.0, |sum, (v, count)| -> Result<f64> {
                Ok(f64::from(u32::try_from(count)?)
                    .mul_add((f64::from(i32::try_from(v)?) - ev).powi(2), sum))
            })?;
        Ok(sum / self.size()?)
    }

    pub fn std_deviation(&self) -> Result<f64> {
        Ok(self.variance()?.sqrt())
    }
}

pub trait Stats {
    fn distribution(&self) -> Result<Distribution>;

    fn min(&self) -> Result<i128> {
        self.distribution()?.min()
    }

    fn max(&self) -> Result<i128> {
        self.distribution()?.max()
    }

    fn expected_value(&self) -> Result<f64> {
        self.distribution()?.expected_value()
    }

    fn variance(&self) -> Result<f64> {
        self.distribution()?.variance()
    }

    fn std_deviation(&self) -> Result<f64> {
        Ok(self.variance()?.sqrt())
    }
}
