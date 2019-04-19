use std::collections::HashMap;

pub fn median(original_numbers: &Vec<i32>) -> i32 {
    let mut numbers = original_numbers.clone();
    numbers.sort_unstable();
    let middle_index = numbers.len() / 2;
    numbers[middle_index]
}

pub fn sum(numbers: &Vec<i32>) -> i32 {
    let mut total = 0;
    for num in numbers.iter() {
        total += num;
    }
    total
}

pub fn mean(numbers: &Vec<i32>) -> f32 {
    let total = sum(numbers);
    total as f32 / numbers.len() as f32
}

pub fn mode(numbers: &Vec<i32>) -> i32 {
    let numbers_map = frequency_map(numbers);
    let max = numbers_map.values().max();
    for (key, val) in numbers_map.iter() {
        if Some(val) == max {
            return *key;
        }
    }
    0
}

pub fn frequency_map(numbers: &Vec<i32>) -> HashMap<i32, i32> {
    let mut numbers_map: HashMap<i32, i32> = HashMap::new();
    for number in numbers.iter() {
        let count = numbers_map.entry(*number).or_insert(0);
        *count += 1;
    }
    numbers_map
}
