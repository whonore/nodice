#![warn(clippy::arithmetic_side_effects)]

use std::{
    ops::{Add, Sub},
    vec,
};

use itertools::Itertools;

use crate::error::{Error, Result};

#[derive(Clone, Debug)]
pub struct Distribution(Vec<i128>);

impl<V: Into<i128>> FromIterator<V> for Distribution {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        Self(iter.into_iter().map(Into::into).collect())
    }
}

impl IntoIterator for Distribution {
    type Item = i128;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Add for Distribution {
    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        self.iter()
            .cartesian_product(rhs)
            .map(|(lhs, rhs)| lhs.checked_add(rhs).ok_or(Error::Overflow))
            .collect::<Result<Self>>()
    }
}

impl Sub for Distribution {
    type Output = Result<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.iter()
            .cartesian_product(rhs)
            .map(|(lhs, rhs)| lhs.checked_sub(rhs).ok_or(Error::Overflow))
            .collect::<Result<Self>>()
    }
}

impl Distribution {
    pub fn iter(&self) -> impl Iterator<Item = i128> {
        self.0.iter().copied()
    }

    pub fn size(&self) -> Result<f64> {
        Ok(f64::from(u32::try_from(self.0.len())?))
    }

    pub fn min(&self) -> Result<i128> {
        self.iter().min().ok_or(Error::EmptyDistribution)
    }

    pub fn max(&self) -> Result<i128> {
        self.iter().max().ok_or(Error::EmptyDistribution)
    }

    pub fn expected_value(&self) -> Result<f64> {
        Ok(f64::from(i32::try_from(self.iter().sum::<i128>())?) / self.size()?)
    }

    pub fn variance(&self) -> Result<f64> {
        let ev = self.expected_value()?;
        let mut sum = 0.0;
        for v in self.iter() {
            sum += (f64::from(i32::try_from(v)?) - ev).powi(2);
        }
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
