use std::collections::HashMap;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut counter = HashMap::new();
        let mut sum = 0;
        let mut ret = 0;

        for j in 0..nums.len() {
            *counter.entry(nums[j]).or_insert(0) += 1;
            sum += nums[j];
            while *counter.get(&nums[j]).unwrap() > 1 {
                *counter.get_mut(&nums[i]).unwrap() -= 1;
                sum -= nums[i];
                i += 1;
            }
            ret = ret.max(sum);
        }

        ret
    }
}
