#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }

    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }

    if let Some(&invalid) = number.iter().find(|&&d| d >= from_base) {
        return Err(Error::InvalidDigit(invalid));
    }

    let initial_number = calculate_in_base(number, from_base);

    Ok(represent_in_base(initial_number, to_base))
}

fn get_digit_significance(index: u32, to_base: u32) -> u32 {
    to_base.pow(index)
}

fn get_max_digits(number: u32, to_base: u32) -> u32 {
    (1..u32::MAX)
        .find(|i| number < get_digit_significance(i.clone(), to_base))
        .unwrap()
}

fn represent_in_base(number: u32, to_base: u32) -> Vec<u32> {
    let mut digits = vec![];
    let mut remainder = number;
    let no_of_digits = get_max_digits(number, to_base);
    println!("no_of_digits {no_of_digits}");
    for index in (0..no_of_digits).rev() {
        let digit_significance = get_digit_significance(index, to_base);
        let quotient = remainder / digit_significance;
        remainder -= quotient * digit_significance;
        digits.push(quotient)
    }
    digits
}

fn calculate_in_base(number: &[u32], from_base: u32) -> u32 {
    number
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, digit)| {
            acc + (digit * from_base.pow(index as u32))
        })
}
