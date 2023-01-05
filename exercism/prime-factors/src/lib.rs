pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = 2..;
    let mut result = vec![];
    let mut rem = n;
    while rem > 1 {
        let factor = factors.next().unwrap();
        while rem % factor == 0 {
            rem /= factor;
            result.push(factor);
        }
    }
    result
}
