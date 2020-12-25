use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let mut count = HashMap::new();

        for &num in &nums {
            *count.entry(num).or_insert(0) += 1;
        }

        nums.sort_unstable_by(|a, b| b.cmp(a));
        nums.sort_by_key(|k| count.get(k).unwrap());

        nums
    }
}
