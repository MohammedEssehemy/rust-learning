use rand::{thread_rng, Rng};
struct Solution {
    original: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self { original: nums }
    }

    fn reset(&self) -> Vec<i32> {
        self.original.clone()
    }

    fn shuffle(&self) -> Vec<i32> {
        let mut random_vec = self.original.clone();
        let vec_len = self.original.len();
        let mut rng = thread_rng();
        for i in 0..vec_len {
            let random_idx = rng.gen_range(i, vec_len);
            let temp = random_vec[random_idx];
            random_vec[random_idx] = random_vec[i];
            random_vec[i] = temp;
        }
        random_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let original = vec![2, 4, 5];
        let solution = Solution::new(original.clone());
        assert_eq!(
            {
                let mut shuffle = solution.shuffle();
                shuffle.sort();
                shuffle
            },
            {
                let mut compare = original.clone();
                compare.sort();
                compare
            }
        );
        assert_eq!(solution.reset(), original);
    }
}
