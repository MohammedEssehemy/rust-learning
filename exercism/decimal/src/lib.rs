// credits
// https://exercism.org/tracks/rust/exercises/decimal/solutions/samidarko
use std::{
    cmp::Ordering,
    ops::{Add, Mul, Neg, Sub},
};

use num_bigint::BigInt;

// https://www.includehelp.com/rust/find-the-lcm.aspx
fn get_lowest_common_multiple(n1: &BigInt, n2: &BigInt) -> BigInt {
    let mut max = n1.max(n2).clone();
    let mut min = n1.min(n2).clone();
    let mut rem = &max % &min;

    while rem != BigInt::from(0) {
        max = min.clone();
        min = rem.clone();
        rem = &max % &min;
    }

    n1 * n2 / min
}

#[derive(Eq, Ord, Debug)]
pub struct Decimal {
    numerator: BigInt,
    denominator: BigInt,
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        let (_, self_numerator, other_numerator) = self.get_common_denominator(other);
        self_numerator == other_numerator
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (_denominator, self_numerator, other_numerator) = self.get_common_denominator(other);
        self_numerator.partial_cmp(&other_numerator)
    }
}

impl Neg for Decimal {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Decimal {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let (denominator, self_numerator, other_numerator) = self.get_common_denominator(&other);
        Self {
            numerator: self_numerator + other_numerator,
            denominator,
        }
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + -other
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            numerator: self.numerator * other.numerator,
            denominator: self.denominator * other.denominator,
        }
    }
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let (left, right) = input.split_once('.').unwrap_or((input, ""));
        let right_trimmed = right.trim_end_matches('0');
        let denominator = BigInt::from(10).pow(right_trimmed.len() as u32);
        let numerator = (left.to_owned() + right_trimmed).parse::<BigInt>().ok()?;
        Some(Self {
            numerator,
            denominator,
        })
    }

    fn get_common_denominator(&self, other: &Self) -> (BigInt, BigInt, BigInt) {
        let denominator = get_lowest_common_multiple(&self.denominator, &other.denominator);
        let self_numerator = &self.numerator * &denominator / &self.denominator;
        let other_numerator = &other.numerator * &denominator / &other.denominator;

        (denominator, self_numerator, other_numerator)
    }
}
