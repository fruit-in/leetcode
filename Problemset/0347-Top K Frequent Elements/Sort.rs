use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counter = HashMap::new();

        for num in nums {
            *counter.entry(num).or_insert(0) += 1;
        }

        let mut counter = counter.iter().collect::<Vec<_>>();

        counter.sort_unstable_by_key(|c| c.1);

        counter
            .iter()
            .rev()
            .take(k as usize)
            .map(|(k, v)| **k)
            .collect()
    }
}
