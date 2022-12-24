pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![];

    (2..)
        .filter(|candidate: &u32| {
            if primes.iter().any(|i| candidate % i == 0) {
                false
            } else {
                primes.push(*candidate);
                true
            }
        })
        .nth(n as usize)
        .unwrap()
}
