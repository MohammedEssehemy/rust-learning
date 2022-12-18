pub fn is_armstrong_number(num: u32) -> bool {
    let stringified = num.to_string();
    let len = stringified.len() as u32;
    let armstrong_result = stringified
        .chars()
        .map(|char| char.to_digit(10).unwrap().pow(len))
        .fold(Some(0_u32), |acc, e| acc.and_then(|sum| sum.checked_add(e)));
    armstrong_result.map_or(false, |res| res == num)
}
