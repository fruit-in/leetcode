use std::collections::HashMap;

impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut longest_length = HashMap::new();

        for &num in &arr {
            longest_length.insert(
                num,
                *longest_length.get(&(num - difference)).unwrap_or(&0) + 1,
            );
        }

        *longest_length.values().max().unwrap()
    }
}
