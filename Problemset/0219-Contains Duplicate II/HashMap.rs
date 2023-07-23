use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            if map.contains_key(&nums[i]) && i <= map[&nums[i]] + k {
                return true;
            } else {
                map.insert(nums[i], i);
            }
        }
        false
    }
}
