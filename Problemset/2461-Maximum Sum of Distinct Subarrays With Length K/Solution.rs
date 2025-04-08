use std::collections::HashMap;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut count = HashMap::new();
        let mut subarray_sum = 0;
        let mut i = 0;
        let mut ret = 0;

        for j in 0..k {
            subarray_sum += nums[j] as i64;
            *count.entry(nums[j]).or_insert(0) += 1;
        }
        if count.len() == k {
            ret = subarray_sum;
        }

        for j in k..nums.len() {
            subarray_sum += nums[j] as i64;
            subarray_sum -= nums[i] as i64;
            *count.entry(nums[j]).or_insert(0) += 1;
            *count.get_mut(&nums[i]).unwrap() -= 1;
            if count[&nums[i]] == 0 {
                count.remove(&nums[i]);
            }
            if count.len() == k {
                ret = ret.max(subarray_sum);
            }
            i += 1;
        }

        ret
    }
}
