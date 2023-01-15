/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    isbn //
        .chars()
        .filter(|&c| c != '-')
        .rev()
        .try_fold((0, 0), |(count, sum), char| {
            let count = count + 1;
            let digit = if count == 1 && char == 'X' {
                10
            } else {
                char.to_digit(10)?
            };
            Some((count, sum + (digit * count)))
        })
        .map_or(false, |(count, sum)| count == 10 && sum % 11 == 0)
}
