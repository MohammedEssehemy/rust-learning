use std::fmt::Display;

/// This trait is for the overflow functionality
trait HasOverflow {
    fn max() -> i32;
    fn min() -> i32;
    fn get_overflow(n: i32) -> (Self, i32)
    where
        Self: Sized + From<i32>,
    {
        let mut remainder = n % Self::max();
        let mut extra_rounds = n / Self::max();
        if remainder < Self::min() {
            remainder += Self::max();
            extra_rounds -= 1;
        }
        (remainder.into(), extra_rounds)
    }
}

/// Used to represent minutes from 0 to 60
#[derive(Debug, PartialEq)]
struct Minutes(i32);
impl Display for Minutes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}
impl From<i32> for Minutes {
    fn from(n: i32) -> Self {
        Self(n)
    }
}
impl HasOverflow for Minutes {
    fn max() -> i32 {
        60
    }
    fn min() -> i32 {
        0
    }
}

/// Used to represent hours from 0 to 24
#[derive(Debug, PartialEq)]
struct Hours(i32);
impl From<i32> for Hours {
    fn from(n: i32) -> Self {
        Self(n)
    }
}
impl HasOverflow for Hours {
    fn max() -> i32 {
        24
    }
    fn min() -> i32 {
        0
    }
}
impl Display for Hours {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: Hours,
    minutes: Minutes,
}

impl Clock {
    pub fn new(h: i32, m: i32) -> Self {
        let (minutes, extra_hours) = Minutes::get_overflow(m);
        let (hours, _extra_days) = Hours::get_overflow(h + extra_hours);
        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours.0, self.minutes.0 + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.hours, self.minutes)
    }
}
