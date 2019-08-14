use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            map.insert(target - nums[i], i);
        }
        for i in 0..nums.len() {
            if let Some(&j) = map.get(&nums[i]) {
                if i != j {
                    return vec![i as i32, j as i32];
                }
            }
        }
        Vec::new()
    }
}
