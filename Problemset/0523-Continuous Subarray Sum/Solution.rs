use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut prefix_mod = 0;
        let mut mod_indices = HashMap::from([(0, -1)]);

        for i in 0..nums.len() {
            prefix_mod = (prefix_mod + nums[i]) % k;

            match mod_indices.get(&prefix_mod) {
                Some(j) if j + 1 < i as i32 => return true,
                None => {
                    mod_indices.insert(prefix_mod, i as i32);
                }
                _ => (),
            }
        }

        false
    }
}
