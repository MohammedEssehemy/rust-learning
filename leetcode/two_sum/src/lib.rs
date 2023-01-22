use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut target_indices = vec![];
    let hash_map = nums
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (index, num)| {
            acc.entry(num).or_insert(vec![]).push(index as i32);
            acc
        });
    for (index, num) in nums.iter().enumerate() {
        let sub = target - num;
        let current_index = index as i32;
        if let Some(other_index) = hash_map
            .get(&sub)
            .and_then(|indices| indices.iter().find(|&i| i != &current_index))
        {
            target_indices = vec![current_index, *other_index];
            break;
        }
    }
    target_indices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }
    #[test]
    fn example_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let res = two_sum(nums, target);
        assert_eq!(res, vec![1, 2]);
    }

    #[test]
    fn example_3() {
        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }
}
