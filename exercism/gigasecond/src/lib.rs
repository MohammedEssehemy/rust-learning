use std::ops::Add;
use time::{ext::NumericalDuration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.add((1e9 as f64).seconds())
}
