pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|n| {
            factors
                .iter()
                .any(|f| n.checked_rem(*f).map(|rem| rem == 0).unwrap_or(false))
        })
        .sum()
}
