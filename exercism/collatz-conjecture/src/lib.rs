pub fn collatz(n: u64) -> Option<u64> {
    let mut res = n;
    let mut steps = 0;
    loop {
        if res == 1 {
            break;
        }
        res = collatz_iter(res)?;
        steps += 1;
    }
    Some(steps)
}

fn collatz_iter(n: u64) -> Option<u64> {
    if n < 1 {
        return None;
    }
    if n % 2 == 0 {
        Some(n / 2)
    } else {
        n.checked_mul(3)?.checked_add(1)
    }
}
