use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for &n in &nums {
            let cnt = map.entry(n).or_insert(0);
            *cnt += 1;
            if *cnt > nums.len() / 2 {
                return n;
            }
        }
        0
    }
}
