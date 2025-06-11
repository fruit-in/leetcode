use std::collections::HashMap;

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut count = HashMap::new();
        let mut max_index = HashMap::new();
        let mut l = 0;
        let mut m = 0;
        let mut ret = 0;

        for r in 0..nums.len() {
            *count.entry(nums[r]).or_insert(0) += 1;
            *max_index.entry(nums[r]).or_insert(r) = r;

            while count.len() > k {
                *count.get_mut(&nums[l]).unwrap() -= 1;
                if *count.get(&nums[l]).unwrap() == 0 {
                    count.remove(&nums[l]);
                }
                l += 1;
            }

            if count.len() == k {
                while m < l || max_index[&nums[m]] != m {
                    m += 1;
                }
                ret += m - l + 1;
            }
        }

        ret as i32
    }
}
