use std::collections::HashMap;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut count = HashMap::new();

        for num in nums {
            *count.entry(num).or_insert(0) += 1;
        }

        count.values().all(|v| v % 2 == 0)
    }
}
