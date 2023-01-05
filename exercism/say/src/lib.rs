pub fn encode(n: u64) -> String {
    let formatter =
        |num: u64, text: &str| format!("{} {} {}", encode(n / num), text, encode(n % num));

    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20 => "twenty".to_string(),
        30 => "thirty".to_string(),
        40 => "forty".to_string(),
        50 => "fifty".to_string(),
        60 => "sixty".to_string(),
        70 => "seventy".to_string(),
        80 => "eighty".to_string(),
        90 => "ninety".to_string(),
        n if n < 99 => format!("{}-{}", encode(10 * (n / 10)), encode(n % 10)),
        n if n < 999 => formatter(10_u64.pow(2), "hundred"),
        n if n < 999_999 => formatter(10_u64.pow(3), "thousand"),
        n if n < 999_999_999 => formatter(10_u64.pow(6), "million"),
        n if n < 999_999_999_999 => formatter(10_u64.pow(9), "billion"),
        n if n < 999_999_999_999_999 => formatter(10_u64.pow(12), "trillion"),
        n if n < 999_999_999_999_999_999 => formatter(10_u64.pow(15), "quadrillion"),
        _ => formatter(10_u64.pow(18), "quintillion"),
    }
    .replace(" zero", "")
}
