pub fn square_iterate(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    (2..=s).fold(1_u64, |acc, _e| acc * 2)
}

pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    2_u64.pow(s - 1)
}

pub fn total() -> u64 {
    (1..=64)
        .map(square) //
        .sum()
}
