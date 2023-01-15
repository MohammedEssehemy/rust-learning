/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut chars = code //
        .chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .peekable();

    let check_digit = match chars.next().unwrap().to_digit(10) {
        Some(c) => c,
        None => return false,
    };

    if chars.peek().is_none() {
        return false;
    }

    chars //
        .enumerate()
        .map(|(i, c)| {
            let mut digit = c.to_digit(10)?;
            if i % 2 == 0 {
                digit *= 2;
            }
            if digit > 9 {
                Some(digit - 9)
            } else {
                Some(digit)
            }
        })
        .sum::<Option<u32>>()
        .map(|s| (1000 - s) % 10)
        .eq(&Some(check_digit))
}
