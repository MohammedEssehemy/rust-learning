pub fn climb_stairs(n: i32) -> i32 {
    match n {
        1 => 1,
        2 => 2,
        _ => {
            let mut n_2 = 1;
            let mut n_1 = 2;
            for _ in 3..n {
                let tmp = n_2;
                n_2 = n_1;
                n_1 = n_1 + tmp;
            }
            n_1 + n_2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_steps() {
        let n = 2;
        assert_eq!(climb_stairs(n), 2);
    }

    #[test]
    fn three_steps() {
        let n = 3;
        assert_eq!(climb_stairs(n), 3);
    }
    #[test]
    fn four_steps() {
        let n = 4;
        assert_eq!(climb_stairs(n), 5);
    }

    #[test]
    fn five_steps() {
        let n = 5;
        assert_eq!(climb_stairs(n), 8);
    }
}
